use std::collections::{HashMap, HashSet};

use anyhow::{Result, bail};

use crate::{
    CELL_SIZE,
    parse::{
        AST,
        AssignNode::{self, Add, Simple, Sub},
        LValueNode::{Head, Variable},
        NumberNode,
        RValueNode::{Get, Number},
        StatementNode::{self, Assign, Put},
        StaticVariableNode,
        TopLevelNode::{self, Statement, While},
        VariableNode::{self, Dynamic, Static},
    },
};

pub struct SemOptimizationOptions {
    pub compress_var_map: bool,
    pub round_var_index: bool,
    pub round_num_literal: bool,
}

impl Default for SemOptimizationOptions {
    #[inline]
    fn default() -> Self {
        Self {
            compress_var_map: true,
            round_var_index: false,
            round_num_literal: true,
        }
    }
}

#[derive(Clone)]
pub struct SemanticInfo {
    pub tape_len: usize,
    pub var_map: HashMap<usize, usize>,
    pub var_map_compressed: bool,
    pub first_cur_dyn_var_only: usize,
    pub warn_big_index: bool,
    pub warn_big_literal: bool,
}

pub struct Semer<'opts> {
    opts: &'opts SemOptimizationOptions,
    info: SemanticInfo,
    cur: usize,
    in_while: bool,
    var_indexes: HashSet<usize>,
    head: usize,
}

impl<'opts> Semer<'opts> {
    pub fn sem(&mut self, ast: &mut AST) -> Result<SemanticInfo> {
        self.info.first_cur_dyn_var_only = ast.root.len();
        self.visit_toplevel_nodes(&mut ast.root)?;
        if self.opts.compress_var_map && self.allow_static_variable() {
            let mut var_indexes = self.var_indexes.iter().collect::<Vec<_>>();
            var_indexes.sort();
            if let Some(last_index) = var_indexes.last()
                && **last_index > var_indexes.len()
            {
                for (i, v) in var_indexes.iter().enumerate() {
                    self.info.var_map.insert(**v, i);
                }
                self.info.var_map_compressed = true;
            }
        }
        Ok(self.info.clone())
    }

    fn visit_toplevel_nodes(&mut self, nodes: &mut [TopLevelNode]) -> Result<()> {
        for node in nodes {
            match node {
                Statement(statement_node) => self.visit_statement(statement_node)?,
                While(while_node) => {
                    self.visit_variable(&mut while_node.condition)?;
                    let in_while = self.in_while;
                    self.in_while = true;
                    self.visit_toplevel_nodes(&mut while_node.content)?;
                    self.in_while = in_while;
                }
            }
            if !self.in_while {
                self.cur += 1;
            }
        }
        Ok(())
    }

    fn visit_statement(&mut self, node: &mut StatementNode) -> Result<()> {
        match node {
            Assign(assign_node) => self.visit_assignment(assign_node)?,
            Put(put_node) => {
                self.visit_variable(&mut put_node.character)?;
            }
        }
        Ok(())
    }

    fn visit_assignment(&mut self, node: &mut AssignNode) -> Result<()> {
        let message_input_to_head = "Cannot assign user's input to head";
        let message_add_input = "Cannot add user's input";
        let message_sub_input = "Cannot subtract user's input";
        match node {
            Simple(operand) => match &mut operand.lvalue {
                Variable(variable_node) => {
                    self.visit_variable(variable_node)?;
                }
                Head(_) => {
                    if self.in_while {
                        self.mark_last_cur();
                    }
                    match &mut operand.rvalue {
                        Number(number_node) => {
                            self.visit_number(number_node);
                            if self.allow_static_variable() {
                                self.head = number_node.value;
                                if self.opts.round_var_index {
                                    self.head %= self.info.tape_len;
                                }
                            }
                        }
                        Get(_) => bail!(message_input_to_head),
                    }
                }
            },
            Add(operand) => {
                let Number(number_node) = &mut operand.rvalue else {
                    bail!(message_add_input);
                };
                match &mut operand.lvalue {
                    Variable(variable_node) => {
                        self.visit_variable(variable_node)?;
                    }
                    Head(_) => {
                        if self.in_while {
                            self.mark_last_cur();
                        } else if self.allow_static_variable() {
                            self.head += number_node.value;
                            if self.opts.round_var_index {
                                self.head %= self.info.tape_len;
                            }
                        }
                    }
                }
            }
            Sub(operand) => {
                let Number(number_node) = &mut operand.rvalue else {
                    bail!(message_sub_input);
                };
                match &mut operand.lvalue {
                    Variable(variable_node) => {
                        self.visit_variable(variable_node)?;
                    }
                    Head(_) => {
                        if self.in_while {
                            self.mark_last_cur();
                        } else if self.allow_static_variable() {
                            self.head -= number_node.value;
                            if self.opts.round_var_index {
                                self.head %= self.info.tape_len;
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn visit_variable(&mut self, variable: &mut VariableNode) -> Result<()> {
        match variable {
            Static(stat_var_node) => {
                if !self.allow_static_variable() {
                    bail!("Cannot use static variable after moving head in while");
                } else {
                    if stat_var_node.index >= self.info.tape_len {
                        self.info.warn_big_index = true;
                    }
                    if self.opts.round_var_index {
                        stat_var_node.index %= self.info.tape_len;
                    }
                    self.var_indexes.insert(stat_var_node.index);
                    if self.var_indexes.len() > self.info.tape_len {
                        bail!("Too many static variables");
                    }
                    Ok(())
                }
            }
            Dynamic(_) => {
                if self.allow_static_variable() && !self.in_while {
                    *variable = Static(StaticVariableNode { index: self.head });
                    self.visit_variable(variable)?;
                }
                Ok(())
            }
        }
    }

    fn visit_number(&mut self, node: &mut NumberNode) {
        if node.value >= CELL_SIZE {
            self.info.warn_big_literal = true;
        }
        if self.opts.round_num_literal {
            node.value %= CELL_SIZE;
        }
    }

    #[inline]
    fn mark_last_cur(&mut self) {
        if self.allow_static_variable() {
            self.info.first_cur_dyn_var_only = self.cur;
        }
    }

    #[inline]
    fn allow_static_variable(&self) -> bool {
        self.cur < self.info.first_cur_dyn_var_only
    }

    #[inline]
    pub fn new(tape_len: usize, opts: &'opts SemOptimizationOptions) -> Self {
        Self {
            opts,
            info: SemanticInfo {
                tape_len,
                var_map: HashMap::new(),
                var_map_compressed: false,
                first_cur_dyn_var_only: 0,
                warn_big_index: false,
                warn_big_literal: false,
            },
            cur: 0,
            in_while: false,
            var_indexes: HashSet::new(),
            head: 0,
        }
    }
}
