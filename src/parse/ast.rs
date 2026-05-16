pub struct AST {
    pub root: Vec<TopLevelNode>,
}

pub enum TopLevelNode {
    While(WhileNode),
    Statement(StatementNode),
}

pub struct WhileNode {
    pub condition: VariableNode,
    pub content: Vec<TopLevelNode>,
}

pub enum StatementNode {
    Assign(AssignNode),
    Put(PutNode),
}

pub enum AssignNode {
    Simple(AssignNodeOperand),
    Add(AssignNodeOperand),
    Sub(AssignNodeOperand),
}

pub struct AssignNodeOperand {
    pub lvalue: LValueNode,
    pub rvalue: RValueNode,
}

pub enum LValueNode {
    Variable(VariableNode),
    Head(HeadNode),
}

pub enum RValueNode {
    Number(NumberNode),
    Get(GetNode),
}

pub struct PutNode {
    pub character: VariableNode,
}

pub struct GetNode;

pub enum VariableNode {
    Static(StaticVariableNode),
    Dynamic(DynamicVariableNode),
}

pub struct StaticVariableNode {
    pub index: usize,
}

pub struct DynamicVariableNode;

pub struct NumberNode {
    pub value: usize,
}

pub struct HeadNode;
