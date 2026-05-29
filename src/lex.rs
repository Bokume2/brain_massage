pub mod token;
pub use token::*;

pub mod lexer;
pub use lexer::*;

use anyhow::Result;

#[inline]
pub fn lex(code: &str) -> Result<Vec<Token>> {
    Lexer::new(code).lex()
}
