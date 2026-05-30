use crate::transpile::{
    BF_CLS, BF_DEC, BF_GET, BF_INC, BF_NXT, BF_OPN, BF_PRV, BF_PUT,
    BFToken::{CLS, DEC, GET, INC, NXT, OPN, PRV, PUT},
};

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

pub fn bf_token_char(token: &BFToken) -> char {
    match token {
        INC => BF_INC,
        DEC => BF_DEC,
        NXT => BF_NXT,
        PRV => BF_PRV,
        GET => BF_GET,
        PUT => BF_PUT,
        OPN => BF_OPN,
        CLS => BF_CLS,
    }
}
