#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BFToken {
    INC,
    DEC,
    NXT,
    PRV,
    GET,
    PUT,
    OPN,
    CLS,
}
