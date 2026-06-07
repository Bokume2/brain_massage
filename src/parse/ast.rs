#[derive(Debug, PartialEq, Eq)]
pub struct AST {
    pub root: Vec<TopLevelNode>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum TopLevelNode {
    While(WhileNode),
    Statement(StatementNode),
}

#[derive(Debug, PartialEq, Eq)]
pub struct WhileNode {
    pub condition: VariableNode,
    pub content: Vec<TopLevelNode>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum StatementNode {
    Assign(AssignNode),
    Put(PutNode),
}

#[derive(Debug, PartialEq, Eq)]
pub enum AssignNode {
    Simple(AssignNodeOperand),
    Add(AssignNodeOperand),
    Sub(AssignNodeOperand),
}

#[derive(Debug, PartialEq, Eq)]
pub struct AssignNodeOperand {
    pub lvalue: LValueNode,
    pub rvalue: RValueNode,
}

#[derive(Debug, PartialEq, Eq)]
pub enum LValueNode {
    Variable(VariableNode),
    Head(HeadNode),
}

#[derive(Debug, PartialEq, Eq)]
pub enum RValueNode {
    Number(NumberNode),
    Get(GetNode),
}

#[derive(Debug, PartialEq, Eq)]
pub struct PutNode {
    pub character: VariableNode,
}

#[derive(Debug, PartialEq, Eq)]
pub struct GetNode;

#[derive(Debug, PartialEq, Eq)]
pub enum VariableNode {
    Static(StaticVariableNode),
    Dynamic(DynamicVariableNode),
}

#[derive(Debug, PartialEq, Eq)]
pub struct StaticVariableNode {
    pub index: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub struct DynamicVariableNode;

#[derive(Debug, PartialEq, Eq)]
pub struct NumberNode {
    pub value: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub struct HeadNode;
