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

pub enum VariableToken {
    Static { index: usize },
    Dynamic,
}

pub enum AssginToken {
    Simple,
    Add,
    Sub,
}

pub enum BracketToken {
    Open,
    Close,
}

pub enum CurlyBracketToken {
    Open,
    Close,
}
