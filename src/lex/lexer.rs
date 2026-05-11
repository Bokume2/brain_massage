use anyhow::{Result, bail};
use regex::Regex;

use crate::lex::{AssginToken, BracketToken, CurlyBracketToken, Token, VariableToken};

pub struct Lexer;

impl Lexer {
    pub fn lex(&self, code: &str) -> Result<Vec<Token>> {
        let put_kw = "put";
        let get_kw = "get";
        let while_kw = "while";
        let head_kw = "head";
        let semicolon = ";";
        let lbracket = "(";
        let rbracket = ")";
        let bracket_ptn = &format!(r"[\{}\{}]", lbracket, rbracket);
        let lcbracket = "{";
        let rcbracket = "}";
        let curly_bracket_ptn = &format!(r"[\{}\{}]", lcbracket, rcbracket);
        let simple_assign_op = "=";
        let add_assign_op = "+=";
        let sub_assign_op = "-=";
        let assign_ptn = &format!(
            r"({})|(\{})|({})",
            simple_assign_op, add_assign_op, sub_assign_op
        );
        let newline_ptn = r"[\r\n]";
        let wsp_ptn = r"[ \t]";
        let decimal_number_ptn = r"[0-9]+";
        let hexadecimal_number_ptn = r"(?i)0x[0-9A-F]+";
        let number_ptn = &format!("({})|({})", hexadecimal_number_ptn, decimal_number_ptn);
        let var_prefix = "v";
        let static_variable_ptn = &format!("{}({})", var_prefix, number_ptn);
        let dynamic_variable_ptn = &format!("{}{}", var_prefix, head_kw);

        let wsp_re = Regex::new(wsp_ptn).unwrap();
        let newline_re = Regex::new(newline_ptn).unwrap();
        let semicolon_re = Regex::new(semicolon).unwrap();
        let put_re = Regex::new(put_kw).unwrap();
        let get_re = Regex::new(get_kw).unwrap();
        let while_re = Regex::new(while_kw).unwrap();
        let head_re = Regex::new(head_kw).unwrap();
        let bracket_re = Regex::new(bracket_ptn).unwrap();
        let curly_bracket_re = Regex::new(curly_bracket_ptn).unwrap();
        let assign_re = Regex::new(assign_ptn).unwrap();
        let number_re = Regex::new(number_ptn).unwrap();
        let static_variable_re = Regex::new(static_variable_ptn).unwrap();
        let dynamic_variable_re = Regex::new(dynamic_variable_ptn).unwrap();
        
        let delimiter_re = Regex::new(r"[ \t\r\n;\(\)\{\}]+").unwrap();

        let mut result = Vec::new();
        let mut cur: usize = 0;
        while cur < code.len() {
            if let Some(_) = Self::cut_token(&wsp_re, code, &mut cur) {
                // nothing to do
            } else if let Some(_) = Self::cut_token(&newline_re, code, &mut cur) {
                result.push(Token::NewLine);
            } else if let Some(_) = Self::cut_token(&semicolon_re, code, &mut cur) {
                result.push(Token::Semicolon);
            } else if let Some(_) = Self::cut_token(&put_re, code, &mut cur) {
                result.push(Token::Put);
            } else if let Some(_) = Self::cut_token(&get_re, code, &mut cur) {
                result.push(Token::Get)
            } else if let Some(_) = Self::cut_token(&while_re, code, &mut cur) {
                result.push(Token::While);
            } else if let Some(_) = Self::cut_token(&head_re, code, &mut cur) {
                result.push(Token::Head);
            } else if let Some(bracket_str) = Self::cut_token(&bracket_re, code, &mut cur) {
                result.push(Token::Bracket(match bracket_str {
                    s if s == lbracket => BracketToken::Open,
                    s if s == rbracket => BracketToken::Close,
                    _ => unreachable!("Unexpected bracket_re match"),
                }));
            } else if let Some(cbracket_str) = Self::cut_token(&curly_bracket_re, code, &mut cur) {
                result.push(Token::CurlyBracket(match cbracket_str {
                    s if s == lcbracket => CurlyBracketToken::Open,
                    s if s == rcbracket => CurlyBracketToken::Close,
                    _ => unreachable!("Unexpected curly_bracket_re match"),
                }));
            } else if let Some(assign_str) = Self::cut_token(&assign_re, code, &mut cur) {
                result.push(Token::Assign(match assign_str {
                    s if s == simple_assign_op => AssginToken::Simple,
                    s if s == add_assign_op => AssginToken::Add,
                    s if s == sub_assign_op => AssginToken::Sub,
                    _ => unreachable!("Unexpected assign_re match"),
                }));
            } else if let Some(num_str) = Self::cut_token(&number_re, code, &mut cur) {
                result.push(Token::Number {
                    value: Self::parse_number(num_str),
                });
            } else if let Some(stat_var_str) = Self::cut_token(&static_variable_re, code, &mut cur) {
                let index = Self::parse_number(&stat_var_str[var_prefix.len()..]);
                result.push(Token::Variable(VariableToken::Static { index }));
            } else if let Some(_) = Self::cut_token(&dynamic_variable_re, code, &mut cur) {
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
            return Some(token_str);
        } else {
            return None;
        }
    }

    fn parse_number(num_str: &str) -> usize {
        if num_str.starts_with("0x") || num_str.starts_with("0X") {
            return usize::from_str_radix(&num_str[2..], 16).unwrap();
        }
        return num_str.parse::<usize>().unwrap();
    }

    pub fn new() -> Lexer {
        Lexer
    }
}

#[cfg(test)]
mod test {
    use crate::lex::{AssginToken, BracketToken, CurlyBracketToken, Lexer, Token, VariableToken};
    use anyhow::Error;

    #[test]
    fn lex_succeed() -> Result<(), Error> {
        let test_code = "v0 = 128
v1 += 0x61; put(v0x1)
while (vhead) {
    vhead -= 1
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
