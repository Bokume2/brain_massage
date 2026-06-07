#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Number { value: usize },
    Variable(VariableToken),
    Head,
    Assign(AssginToken),
    Bracket(BracketToken),
    CurlyBracket(CurlyBracketToken),
    While,
    Put,
    Get,
    Semicolon,
    NewLine,
}

#[derive(Debug, PartialEq, Eq)]
pub enum VariableToken {
    Static { index: usize },
    Dynamic,
}

#[derive(Debug, PartialEq, Eq)]
pub enum AssginToken {
    Simple,
    Add,
    Sub,
}

#[derive(Debug, PartialEq, Eq)]
pub enum BracketToken {
    Open,
    Close,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CurlyBracketToken {
    Open,
    Close,
}
