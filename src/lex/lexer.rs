use std::sync::LazyLock;

use anyhow::{Result, bail};
use regex::Regex;

use crate::lex::{AssginToken, BracketToken, CurlyBracketToken, Token, VariableToken};

pub const PUT_KW: &str = "put";
pub const GET_KW: &str = "get";
pub const WHILE_KW: &str = "while";
pub const HEAD_KW: &str = "head";
pub const TAPE_KW: &str = "tape";

pub const SEMICOLON: &str = ";";
pub const LBRACKET: &str = "(";
pub const RBRACKET: &str = ")";
pub const LCBRACKET: &str = "{";
pub const RCBRACKET: &str = "}";

pub const SIMPLE_ASSIGN_OP: &str = "=";
pub const ADD_ASSIGN_OP: &str = "+=";
pub const SUB_ASSIGN_OP: &str = "-=";

pub const VAR_PREFIX: &str = "v";

const NUMBER_PTN: &str = r"(?i)(0x[0-9A-F]+)|([0-9]+)";
const DELIMITER_PTN: &str = r"[ \t\r\n;\(\)\[\]\{\}]+";

pub struct Lexer;

impl Lexer {
    pub fn lex(&self, code: &str) -> Result<Vec<Token>> {
        const INDEX1: &str = "index1";
        const INDEX2: &str = "index2";

        let wsp_re = {
            static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[ \t]").unwrap());
            LazyLock::force(&RE)
        };
        let newline_re = {
            static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[\r\n]").unwrap());
            LazyLock::force(&RE)
        };
        let semicolon_re = {
            static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(SEMICOLON).unwrap());
            LazyLock::force(&RE)
        };
        let put_re = {
            static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(PUT_KW).unwrap());
            LazyLock::force(&RE)
        };
        let get_re = {
            static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(GET_KW).unwrap());
            LazyLock::force(&RE)
        };
        let while_re = {
            static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(WHILE_KW).unwrap());
            LazyLock::force(&RE)
        };
        let head_re = {
            static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(HEAD_KW).unwrap());
            LazyLock::force(&RE)
        };
        let bracket_re = {
            static RE: LazyLock<Regex> =
                LazyLock::new(|| Regex::new(&format!(r"[\{}\{}]", LBRACKET, RBRACKET)).unwrap());
            LazyLock::force(&RE)
        };
        let curly_bracket_re = {
            static RE: LazyLock<Regex> =
                LazyLock::new(|| Regex::new(&format!(r"[\{}\{}]", LCBRACKET, RCBRACKET)).unwrap());
            LazyLock::force(&RE)
        };
        let assign_re = {
            static RE: LazyLock<Regex> = LazyLock::new(|| {
                Regex::new(&format!(
                    r"({})|(\{})|({})",
                    SIMPLE_ASSIGN_OP, ADD_ASSIGN_OP, SUB_ASSIGN_OP
                ))
                .unwrap()
            });
            LazyLock::force(&RE)
        };
        let number_re = {
            static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(NUMBER_PTN).unwrap());
            LazyLock::force(&RE)
        };
        let static_variable_re = {
            static RE: LazyLock<Regex> = LazyLock::new(|| {
                Regex::new(&format!(
                    r"({}(?P<{}>{}))|({}\[(?P<{}>{})\])",
                    VAR_PREFIX, INDEX1, NUMBER_PTN, TAPE_KW, INDEX2, NUMBER_PTN
                ))
                .unwrap()
            });
            LazyLock::force(&RE)
        };
        let dynamic_variable_re = {
            static RE: LazyLock<Regex> = LazyLock::new(|| {
                Regex::new(&format!(
                    r"({}{})|({}\[{}\])",
                    VAR_PREFIX, HEAD_KW, TAPE_KW, HEAD_KW
                ))
                .unwrap()
            });
            LazyLock::force(&RE)
        };

        let delimiter_re = {
            static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(DELIMITER_PTN).unwrap());
            LazyLock::force(&RE)
        };

        let mut result = Vec::new();
        let mut cur: usize = 0;
        while cur < code.len() {
            if Self::cut_token(wsp_re, code, &mut cur).is_some() {
                // nothing to do
            } else if Self::cut_token(newline_re, code, &mut cur).is_some() {
                result.push(Token::NewLine);
            } else if Self::cut_token(semicolon_re, code, &mut cur).is_some() {
                result.push(Token::Semicolon);
            } else if Self::cut_token(put_re, code, &mut cur).is_some() {
                result.push(Token::Put);
            } else if Self::cut_token(get_re, code, &mut cur).is_some() {
                result.push(Token::Get)
            } else if Self::cut_token(while_re, code, &mut cur).is_some() {
                result.push(Token::While);
            } else if Self::cut_token(head_re, code, &mut cur).is_some() {
                result.push(Token::Head);
            } else if let Some(bracket_str) = Self::cut_token(bracket_re, code, &mut cur) {
                result.push(Token::Bracket(match bracket_str {
                    s if s == LBRACKET => BracketToken::Open,
                    s if s == RBRACKET => BracketToken::Close,
                    _ => unreachable!("Unexpected bracket_re match"),
                }));
            } else if let Some(cbracket_str) = Self::cut_token(curly_bracket_re, code, &mut cur) {
                result.push(Token::CurlyBracket(match cbracket_str {
                    s if s == LCBRACKET => CurlyBracketToken::Open,
                    s if s == RCBRACKET => CurlyBracketToken::Close,
                    _ => unreachable!("Unexpected curly_bracket_re match"),
                }));
            } else if let Some(assign_str) = Self::cut_token(assign_re, code, &mut cur) {
                result.push(Token::Assign(match assign_str {
                    s if s == SIMPLE_ASSIGN_OP => AssginToken::Simple,
                    s if s == ADD_ASSIGN_OP => AssginToken::Add,
                    s if s == SUB_ASSIGN_OP => AssginToken::Sub,
                    _ => unreachable!("Unexpected assign_re match"),
                }));
            } else if let Some(num_str) = Self::cut_token(number_re, code, &mut cur) {
                result.push(Token::Number {
                    value: Self::parse_number(num_str),
                });
            } else if let Some(stat_var_str) = Self::cut_token(static_variable_re, code, &mut cur) {
                let caps = static_variable_re.captures(stat_var_str).unwrap();
                let index = caps
                    .name(INDEX1)
                    .unwrap_or_else(|| caps.name(INDEX2).unwrap())
                    .as_str();
                let index = Self::parse_number(index);
                result.push(Token::Variable(VariableToken::Static { index }));
            } else if Self::cut_token(dynamic_variable_re, code, &mut cur).is_some() {
                result.push(Token::Variable(VariableToken::Dynamic));
            } else {
                let token_like = delimiter_re.splitn(&code[cur..], 2).next().unwrap();
                let message = format!("Unknown token \"{}\"", token_like);
                bail!(message);
            }
        }
        Ok(result)
    }

    fn cut_token<'a>(re: &Regex, code: &'a str, cur: &mut usize) -> Option<&'a str> {
        let m = re.find_at(code, *cur)?;
        if m.start() == *cur {
            let token_str = m.as_str();
            *cur += token_str.len();
            Some(token_str)
        } else {
            None
        }
    }

    fn parse_number(num_str: &str) -> usize {
        if num_str.starts_with("0x") || num_str.starts_with("0X") {
            return usize::from_str_radix(&num_str[2..], 16).unwrap();
        }
        num_str.parse::<usize>().unwrap()
    }

    pub fn new() -> Lexer {
        Lexer
    }
}

impl Default for Lexer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::Error;

    #[test]
    fn lex_succeed() -> Result<(), Error> {
        let test_code = "v0 = 128
v1 += 0x61; put(v0x1)
while (vhead) {
    tape[head] -= 1
    head += 0
}";
        let expected_tokens = vec![
            Token::Variable(VariableToken::Static { index: 0 }),
            Token::Assign(AssginToken::Simple),
            Token::Number { value: 128 },
            Token::NewLine,
            Token::Variable(VariableToken::Static { index: 1 }),
            Token::Assign(AssginToken::Add),
            Token::Number { value: 0x61 },
            Token::Semicolon,
            Token::Put,
            Token::Bracket(BracketToken::Open),
            Token::Variable(VariableToken::Static { index: 0x1 }),
            Token::Bracket(BracketToken::Close),
            Token::NewLine,
            Token::While,
            Token::Bracket(BracketToken::Open),
            Token::Variable(VariableToken::Dynamic),
            Token::Bracket(BracketToken::Close),
            Token::CurlyBracket(CurlyBracketToken::Open),
            Token::NewLine,
            Token::Variable(VariableToken::Dynamic),
            Token::Assign(AssginToken::Sub),
            Token::Number { value: 1 },
            Token::NewLine,
            Token::Head,
            Token::Assign(AssginToken::Add),
            Token::Number { value: 0 },
            Token::NewLine,
            Token::CurlyBracket(CurlyBracketToken::Close),
        ];

        assert_eq!(expected_tokens, Lexer::new().lex(test_code)?);

        Ok(())
    }

    #[test]
    fn lex_of_illegal_code_fail() {
        let illegal_code = "hoge";
        assert!(Lexer::new().lex(illegal_code).is_err())
    }
}
