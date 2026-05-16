use crate::parse::{AST, AssignNode, LValueNode, RValueNode, StatementNode, TopLevelNode, VariableNode, WhileNode};

struct TranspileContext {
    tape_len: usize,
    head: usize,
}

impl Default for TranspileContext {
    fn default() -> Self {
        TranspileContext { tape_len: 4096, head: 0 }
    }
}

pub struct Transpiler;

impl Transpiler {
    pub fn transpile(&self, ast: &AST) -> String {
        let mut ctx = TranspileContext::default();
        Self::transpile_toplevel(&ast.root, &mut ctx)
    }

    fn transpile_toplevel(toplevel: &Vec<TopLevelNode>, ctx: &mut TranspileContext) -> String {
        let mut result = String::new();
        for node in toplevel {
            result += match node {
                TopLevelNode::Statement(statement_node) => Self::transpile_statement(statement_node, ctx),
                TopLevelNode::While(while_node) => Self::transpile_while(while_node, ctx),
            }.as_str();
        }
        result
    }

    fn transpile_statement(statement_node: &StatementNode, ctx: &mut TranspileContext) -> String {
        match statement_node {
            StatementNode::Assign(assign_node) => Self::transpile_assgin(assign_node, ctx),
            StatementNode::Put(put_node) => Self::transpile_variable(&put_node.character, ctx) + ".",
        }
    }

    fn transpile_while(while_node: &WhileNode, ctx: &mut TranspileContext) -> String {
        let mut result = Self::transpile_variable(&while_node.condition, ctx);
        result += "[";
        result += Self::transpile_toplevel(&while_node.content, ctx).as_str();
        result += Self::transpile_variable(&while_node.condition, ctx).as_str();
        result += "]";
        result
    }

    fn transpile_assgin(assign_node: &AssignNode, ctx: &mut TranspileContext) -> String {
        match assign_node {
            AssignNode::Simple(operand) => {
                match &operand.lvalue {
                    LValueNode::Variable(variable) => {
                        Self::transpile_variable(&variable, ctx) + "[-]" + match &operand.rvalue {
                            RValueNode::Number(number) => String::from("+").repeat(number.value),
                            RValueNode::Get(_) => String::from(","),
                        }.as_str()
                    }
                    LValueNode::Head(_) => {
                        let RValueNode::Number(number) = &operand.rvalue else {
                            panic!("cannot assign user's input to head");
                        };
                        let diff = (number.value % ctx.tape_len) as isize - ctx.head as isize;
                        Self::move_head(diff)
                    },
                }
            }
            AssignNode::Add(operand) => {
                match &operand.lvalue {
                    LValueNode::Variable(variable) => {
                        let RValueNode::Number(number) = &operand.rvalue else {
                            panic!("cannot add user's input");
                        };
                        Self::transpile_variable(&variable, ctx) + String::from("+").repeat(number.value).as_str()
                    }
                    LValueNode::Head(_) => {
                        let RValueNode::Number(number) = &operand.rvalue else {
                            panic!("cannot add user's input");
                        };
                        Self::move_head(number.value as isize)
                    }
                }
            }
            AssignNode::Sub(operand) => {
                match &operand.lvalue {
                    LValueNode::Variable(variable) => {
                        let RValueNode::Number(number) = &operand.rvalue else {
                            panic!("cannot subtract user's input");
                        };
                        Self::transpile_variable(&variable, ctx) + String::from("-").repeat(number.value).as_str()
                    }
                    LValueNode::Head(_) => {
                        let RValueNode::Number(number) = &operand.rvalue else {
                            panic!("cannot add user's input");
                        };
                        Self::move_head(-(number.value as isize))
                    }
                }
            }
        }
    }

    fn transpile_variable(variable: &VariableNode, ctx: &mut TranspileContext) -> String {
        match variable {
            VariableNode::Static(stat_var_node) => {
                let diff = (stat_var_node.index % ctx.tape_len) as isize - ctx.head as isize;
                ctx.head = stat_var_node.index % ctx.tape_len;
                Self::move_head(diff)
            }
            VariableNode::Dynamic(_) => String::from("")
        }
    }

    fn move_head(diff: isize) -> String {

                if diff >= 0 {
                    String::from(">").repeat(diff as usize)
                } else {
                    String::from("<").repeat((-diff) as usize)
                }
    }

    pub fn new() -> Self {
        Transpiler
    }
}

impl Default for Transpiler {
    fn default() -> Self {
        Self::new()
    }
}
