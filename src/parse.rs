pub mod ast;
pub use ast::*;

pub mod parser;
pub use parser::*;

use anyhow::Result;

use crate::lex::Token;

#[inline]
pub fn parse(tokens: &[Token]) -> Result<AST> {
    Parser::new(tokens).parse()
}
