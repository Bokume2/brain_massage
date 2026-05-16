#[derive(Debug, PartialEq)]
pub struct AST {
    pub root: Vec<TopLevelNode>,
}

#[derive(Debug, PartialEq)]
pub enum TopLevelNode {
    While(WhileNode),
    Statement(StatementNode),
}

#[derive(Debug, PartialEq)]
pub struct WhileNode {
    pub condition: VariableNode,
    pub content: Vec<TopLevelNode>,
}

#[derive(Debug, PartialEq)]
pub enum StatementNode {
    Assign(AssignNode),
    Put(PutNode),
}

#[derive(Debug, PartialEq)]
pub enum AssignNode {
    Simple(AssignNodeOperand),
    Add(AssignNodeOperand),
    Sub(AssignNodeOperand),
}

#[derive(Debug, PartialEq)]
pub struct AssignNodeOperand {
    pub lvalue: LValueNode,
    pub rvalue: RValueNode,
}

#[derive(Debug, PartialEq)]
pub enum LValueNode {
    Variable(VariableNode),
    Head(HeadNode),
}

#[derive(Debug, PartialEq)]
pub enum RValueNode {
    Number(NumberNode),
    Get(GetNode),
}

#[derive(Debug, PartialEq)]
pub struct PutNode {
    pub character: VariableNode,
}

#[derive(Debug, PartialEq)]
pub struct GetNode;

#[derive(Debug, PartialEq)]
pub enum VariableNode {
    Static(StaticVariableNode),
    Dynamic(DynamicVariableNode),
}

#[derive(Debug, PartialEq)]
pub struct StaticVariableNode {
    pub index: usize,
}

#[derive(Debug, PartialEq)]
pub struct DynamicVariableNode;

#[derive(Debug, PartialEq)]
pub struct NumberNode {
    pub value: usize,
}

#[derive(Debug, PartialEq)]
pub struct HeadNode;
