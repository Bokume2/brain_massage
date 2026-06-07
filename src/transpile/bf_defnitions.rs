#[derive(Debug, PartialEq, Clone, Copy)]
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
