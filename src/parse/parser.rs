use anyhow::{Result, bail};

use crate::{
    lex::{AssginToken, BracketToken, CurlyBracketToken, LCBRACKET, Token, VariableToken},
    parse::{
        AST, AssignNode, AssignNodeOperand, DynamicVariableNode, GetNode, HeadNode, LValueNode,
        NumberNode, PutNode, RValueNode, StatementNode, StaticVariableNode, TopLevelNode,
        VariableNode, WhileNode,
    },
};

pub struct Parser<'source> {
    tokens: &'source [Token],
    cur: usize,
}

impl<'source> Parser<'source> {
    pub fn parse(&mut self) -> Result<AST> {
        let root = self.parse_top_level_nodes()?;
        Ok(AST { root })
    }

    fn parse_top_level_nodes(&mut self) -> Result<Vec<TopLevelNode>> {
        let mut result = Vec::new();
        while self.cur < self.tokens.len() {
            let token = self.tokens.get(self.cur).unwrap();
            match token {
                Token::NewLine | Token::Semicolon => {
                    self.cur += 1;
                    continue;
                }
                Token::Variable(_) | Token::Head | Token::Put => {
                    result.push(TopLevelNode::Statement(self.parse_statement()?))
                }
                Token::While => result.push(TopLevelNode::While(self.parse_while()?)),
                Token::CurlyBracket(cbtoken) => match cbtoken {
                    CurlyBracketToken::Close => return Ok(result),
                    CurlyBracketToken::Open => bail!("Unexpected \"{}\"", LCBRACKET),
                },
                other => bail!("Unexpected token {:?}", other),
            }
        }
        Ok(result)
    }

    fn parse_statement(&mut self) -> Result<StatementNode> {
        let statement_node = match self.tokens.get(self.cur).unwrap() {
            Token::Put => StatementNode::Put(self.parse_put()?),
            Token::Variable(_) | Token::Head => StatementNode::Assign(self.parse_assign()?),
            other => bail!("Expected statement, found {:?}", other),
        };
        let terminator = self.tokens.get(self.cur).unwrap_or(&Token::NewLine);
        if *terminator != Token::Semicolon
            && *terminator != Token::NewLine
            && *terminator != Token::CurlyBracket(CurlyBracketToken::Close)
        {
            bail!("statement must be terminated by \";\" or newline");
        }
        Ok(statement_node)
    }

    fn parse_while(&mut self) -> Result<WhileNode> {
        if *self.get_token_without_nl()? != Token::While {
            panic!("Illegal usage of parse_while");
        }
        if *self.get_token_without_nl()? != Token::Bracket(BracketToken::Open) {
            bail!("\"while\" must be followed by \"(\"");
        }
        let Token::Variable(condition_token) = self.get_token_without_nl()? else {
            bail!(
                "expected variable, found {:?}",
                self.tokens.get(self.cur - 1).unwrap()
            );
        };
        let condition = self.parse_variable(condition_token);
        if *self.get_token_without_nl()? != Token::Bracket(BracketToken::Close) {
            bail!("bracket mismatch, not enough \")\"");
        }
        if *self.get_token_without_nl()? != Token::CurlyBracket(CurlyBracketToken::Open) {
            bail!("while must have loop content starts with \"{{\"");
        }
        let content = self.parse_top_level_nodes()?;
        if *self.get_token_without_nl()? != Token::CurlyBracket(CurlyBracketToken::Close) {
            bail!("bracket mismatch, not enough \"}}\"");
        }
        Ok(WhileNode { condition, content })
    }

    fn parse_put(&mut self) -> Result<PutNode> {
        if *self.get_token_without_nl()? != Token::Put {
            panic!("illegal usage of parse_put");
        }
        if *self.get_token_without_nl()? != Token::Bracket(BracketToken::Open) {
            bail!("\"put\" must be followed by \"(\"");
        }
        let Token::Variable(char_token) = self.get_token_without_nl()? else {
            bail!(
                "expected variable, found {:?}",
                self.tokens.get(self.cur - 1).unwrap()
            );
        };
        let character = self.parse_variable(char_token);
        if *self.get_token_without_nl()? != Token::Bracket(BracketToken::Close) {
            bail!("bracket mismatch, not enough \")\"");
        }
        Ok(PutNode { character })
    }

    fn parse_get(&mut self) -> Result<GetNode> {
        if *self.get_token_without_nl()? != Token::Get {
            panic!("Illegal usage of parse_get");
        }
        if *self.get_token_without_nl()? != Token::Bracket(BracketToken::Open) {
            bail!("\"get\" must be followed by \"(\"");
        }
        if *self.get_token_without_nl()? != Token::Bracket(BracketToken::Close) {
            bail!("bracket mismatch, not enough \")\"");
        }
        Ok(GetNode)
    }

    fn parse_assign(&mut self) -> Result<AssignNode> {
        let lvalue = match self.get_token_without_nl()? {
            Token::Variable(var_token) => LValueNode::Variable(self.parse_variable(var_token)),
            Token::Head => LValueNode::Head(HeadNode),
            other => bail!("expected variable or head, found {:?}", other),
        };
        let Token::Assign(assign_token) = self.get_token_without_nl()? else {
            bail!(
                "expected assignment, found {:?}",
                self.tokens.get(self.cur - 1).unwrap()
            )
        };
        let rvalue = match self.get_token_without_nl()? {
            Token::Number { value } => RValueNode::Number(NumberNode { value: *value }),
            Token::Get => {
                self.cur -= 1;
                RValueNode::Get(self.parse_get()?)
            }
            other => bail!("expected number or get, found {:?}", other),
        };
        let operand = AssignNodeOperand { lvalue, rvalue };
        Ok(match assign_token {
            AssginToken::Simple => AssignNode::Simple(operand),
            AssginToken::Add => AssignNode::Add(operand),
            AssginToken::Sub => AssignNode::Sub(operand),
        })
    }

    fn parse_variable(&mut self, var_token: &VariableToken) -> VariableNode {
        match var_token {
            VariableToken::Static { index } => {
                VariableNode::Static(StaticVariableNode { index: *index })
            }
            VariableToken::Dynamic => VariableNode::Dynamic(DynamicVariableNode),
        }
    }

    fn get_token_without_nl(&mut self) -> Result<&'source Token> {
        loop {
            let Some(token) = self.tokens.get(self.cur) else {
                bail!("Unexpected EOF")
            };
            match token {
                Token::NewLine => {
                    self.cur += 1;
                    continue;
                }
                other => {
                    self.cur += 1;
                    return Ok(other);
                }
            }
        }
    }

    #[inline]
    pub const fn new(tokens: &'source [Token]) -> Self {
        Self { tokens, cur: 0 }
    }
}

#[cfg(test)]
mod test {
    use crate::parse::{
        AssignNode::{Add, Simple, Sub},
        LValueNode::{Head, Variable},
        RValueNode::{Get, Number},
        StatementNode::{Assign, Put},
        TopLevelNode::{Statement, While},
        VariableNode::{Dynamic, Static},
    };

    use super::*;
    use anyhow::Error;

    #[test]
    fn parse_succeed() -> Result<(), Error> {
        let test_tokens = [
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
            Token::Variable(VariableToken::Dynamic),
            Token::Assign(AssginToken::Simple),
            Token::Get,
            Token::Bracket(BracketToken::Open),
            Token::Bracket(BracketToken::Close),
            Token::CurlyBracket(CurlyBracketToken::Close),
        ];

        let expected_ast = AST {
            root: vec![
                Statement(Assign(Simple(AssignNodeOperand {
                    lvalue: Variable(Static(StaticVariableNode { index: 0 })),
                    rvalue: Number(NumberNode { value: 128 }),
                }))),
                Statement(Assign(Add(AssignNodeOperand {
                    lvalue: Variable(Static(StaticVariableNode { index: 1 })),
                    rvalue: Number(NumberNode { value: 0x61 }),
                }))),
                Statement(Put(PutNode {
                    character: Static(StaticVariableNode { index: 0x1 }),
                })),
                While(WhileNode {
                    condition: Dynamic(DynamicVariableNode),
                    content: vec![
                        Statement(Assign(Sub(AssignNodeOperand {
                            lvalue: Variable(Dynamic(DynamicVariableNode)),
                            rvalue: Number(NumberNode { value: 1 }),
                        }))),
                        Statement(Assign(Add(AssignNodeOperand {
                            lvalue: Head(HeadNode),
                            rvalue: Number(NumberNode { value: 0 }),
                        }))),
                        Statement(Assign(Simple(AssignNodeOperand {
                            lvalue: Variable(Dynamic(DynamicVariableNode)),
                            rvalue: Get(GetNode),
                        }))),
                    ],
                }),
            ],
        };
        let ast = Parser::new(&test_tokens).parse()?;
        assert_eq!(ast, expected_ast);

        Ok(())
    }
}
