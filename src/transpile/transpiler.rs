use anyhow::{Result, anyhow, bail};

use crate::{
    parse::{
        AST,
        AssignNode::{self, Add, Simple, Sub},
        LValueNode::{Head, Variable},
        RValueNode::{Get, Number},
        StatementNode::{self, Assign, Put},
        TopLevelNode::{self, Statement, While},
        VariableNode::{self, Dynamic, Static},
        WhileNode,
    },
    sem::SemanticInfo,
    transpile::BFToken::{self, CLS, DEC, GET, INC, NXT, OPN, PRV, PUT},
};

pub struct TranspileOptimizationOptions {
    pub number_compaction: bool,
    pub head_move_around_compaction: bool,
}

impl Default for TranspileOptimizationOptions {
    #[inline]
    fn default() -> Self {
        Self {
            number_compaction: true,
            head_move_around_compaction: true,
        }
    }
}

pub struct Transpiler<'input> {
    ast: &'input AST,
    info: &'input SemanticInfo,
    opts: &'input TranspileOptimizationOptions,
    cur: usize,
    head: usize,
    in_while: bool,
    used_vars: Vec<bool>,
}

impl<'input> Transpiler<'input> {
    const MESSAGE_NOT_ALLOWED_STATIC_VAR: &'static str =
        "Cannot use static variable after moving head in while";
    const MESSAGE_INPUT_TO_HEAD: &'static str = "Cannot add user's input";
    const MESSAGE_ADD_INPUT: &'static str = "Cannot add user's input";
    const MESSAGE_SUB_INPUT: &'static str = "Cannot subtract user's input";

    pub fn transpile(&mut self) -> Result<Vec<BFToken>> {
        self.transpile_toplevel_nodes(&self.ast.root)
    }

    fn transpile_toplevel_nodes(&mut self, nodes: &[TopLevelNode]) -> Result<Vec<BFToken>> {
        let mut result = Vec::new();
        for node in nodes {
            result.extend(match node {
                Statement(staement_node) => self.transpile_statement(staement_node)?,
                While(while_node) => {
                    let in_while = self.in_while;
                    self.in_while = true;
                    let result_while = self.transpile_while(while_node)?;
                    self.in_while = in_while;
                    result_while
                }
            });
            if !self.in_while {
                self.cur += 1;
            }
        }
        Ok(result)
    }

    fn transpile_statement(&mut self, node: &StatementNode) -> Result<Vec<BFToken>> {
        Ok(match node {
            Assign(assign_node) => self.transpile_assign(assign_node)?,
            Put(put_node) => [self.transpile_variable(&put_node.character)?, vec![PUT]].concat(),
        })
    }

    fn transpile_while(&mut self, node: &WhileNode) -> Result<Vec<BFToken>> {
        Ok([
            self.transpile_variable(&node.condition)?,
            vec![OPN],
            self.transpile_toplevel_nodes(&node.content)?,
            self.transpile_variable(&node.condition)?,
            vec![CLS],
        ]
        .concat())
    }

    fn transpile_assign(&mut self, node: &AssignNode) -> Result<Vec<BFToken>> {
        Ok(match node {
            Simple(operand) => match &operand.lvalue {
                Variable(var_node) => [
                    self.transpile_variable(var_node)?,
                    match &operand.rvalue {
                        Number(num_node) => [
                            vec![OPN, DEC, CLS],
                            self.transpile_number(num_node.value, true),
                        ]
                        .concat(),
                        Get(_) => vec![GET],
                    },
                ]
                .concat(),
                Head(_) => match &operand.rvalue {
                    Number(number_node) => self.set_head(number_node.value),
                    Get(_) => bail!(Self::MESSAGE_INPUT_TO_HEAD),
                },
            },
            Add(operand) => {
                let Number(num_node) = &operand.rvalue else {
                    bail!(Self::MESSAGE_ADD_INPUT);
                };
                match &operand.lvalue {
                    Variable(var_node) => [
                        self.transpile_variable(var_node)?,
                        self.transpile_number(num_node.value, true),
                    ]
                    .concat(),
                    Head(_) => self.move_head(num_node.value, true),
                }
            }
            Sub(operand) => {
                let Number(num_node) = &operand.rvalue else {
                    bail!(Self::MESSAGE_SUB_INPUT);
                };
                match &operand.lvalue {
                    Variable(var_node) => [
                        self.transpile_variable(var_node)?,
                        self.transpile_number(num_node.value, false),
                    ]
                    .concat(),
                    Head(_) => self.move_head(num_node.value, false),
                }
            }
        })
    }

    fn transpile_variable(&mut self, node: &VariableNode) -> Result<Vec<BFToken>> {
        Ok(match node {
            Static(static_var_node) => {
                if !self.allow_static_variable() {
                    bail!(Self::MESSAGE_NOT_ALLOWED_STATIC_VAR);
                }
                let index = if self.info.var_map_compressed {
                    *self
                        .info
                        .var_map
                        .get(&static_var_node.index)
                        .ok_or_else(|| {
                            anyhow!("Illegal semantic info: Variable mapping is broken")
                        })?
                } else {
                    static_var_node.index
                };
                self.set_head(index)
            }
            Dynamic(_) => vec![],
        })
    }

    fn transpile_number(&mut self, value: usize, positive: bool) -> Vec<BFToken> {
        let inst = if positive { INC } else { DEC };
        if !self.allow_static_variable() || !self.opts.number_compaction {
            return vec![inst; value];
        }
        self.used_vars[self.head] = true;
        //TODO: +または-の繰り返しを短縮する最適化を実装
        todo!("Please disable inc/dec repetition compaction");
    }

    fn move_head(&mut self, diff: usize, positive: bool) -> Vec<BFToken> {
        if !self.allow_static_variable() || !self.opts.head_move_around_compaction {
            if positive {
                self.head += diff;
                return vec![NXT; diff];
            } else {
                self.head -= diff;
                return vec![PRV; diff];
            }
        }
        let old_head = self.head;
        if positive {
            self.head += diff;
        } else {
            while self.head < diff {
                self.head += self.info.tape_len;
            }
            self.head -= diff;
        }
        self.head %= self.info.tape_len;
        vec![if self.head >= old_head { NXT } else { PRV }; old_head.abs_diff(self.head)]
    }

    #[inline]
    fn set_head(&mut self, head: usize) -> Vec<BFToken> {
        self.move_head(head.abs_diff(self.head), head >= self.head)
    }

    #[inline]
    fn allow_static_variable(&self) -> bool {
        self.cur < self.info.first_cur_dyn_var_only
    }

    #[inline]
    pub fn new(
        ast: &'input AST,
        info: &'input SemanticInfo,
        opts: &'input TranspileOptimizationOptions,
    ) -> Self {
        Self {
            ast,
            info,
            opts,
            cur: 0,
            head: 0,
            in_while: false,
            used_vars: vec![false; info.tape_len],
        }
    }
}
