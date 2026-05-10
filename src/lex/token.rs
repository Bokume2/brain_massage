#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum VariableToken {
    Static { index: usize },
    Dynamic,
}

#[derive(Debug, PartialEq)]
pub enum AssginToken {
    Simple,
    Add,
    Sub,
}

#[derive(Debug, PartialEq)]
pub enum BracketToken {
    Open,
    Close,
}

#[derive(Debug, PartialEq)]
pub enum CurlyBracketToken {
    Open,
    Close,
}
