use anyhow::{Ok, Result, bail};

use crate::{
    lex::{AssginToken, BracketToken, CurlyBracketToken, RCBRACKET, Token, VariableToken},
    parse::{
        AST, AssignNode, AssignNodeOperand, DynamicVariableNode, GetNode, HeadNode, LValueNode,
        NumberNode, PutNode, RValueNode, StatementNode, StaticVariableNode, TopLevelNode,
        VariableNode, WhileNode,
    },
};

#[derive(Default)]
struct ParseContext {
    cur: usize,
}

pub struct Parser;

impl Parser {
    pub fn parse(&self, tokens: &[Token]) -> Result<AST> {
        let mut ctx = ParseContext::default();
        let root = Self::parse_top_level_nodes(tokens, &mut ctx)?;
        Ok(AST { root })
    }

    fn parse_top_level_nodes(
        tokens: &[Token],
        ctx: &mut ParseContext,
    ) -> Result<Vec<TopLevelNode>> {
        let mut result = Vec::new();
        while ctx.cur < tokens.len() {
            let token = tokens.get(ctx.cur).unwrap();
            match token {
                Token::NewLine | Token::Semicolon => {
                    ctx.cur += 1;
                    continue;
                }
                Token::Variable(_) | Token::Head | Token::Put => {
                    result.push(TopLevelNode::Statement(Self::parse_statement(tokens, ctx)?))
                }
                Token::While => result.push(TopLevelNode::While(Self::parse_while(tokens, ctx)?)),
                Token::CurlyBracket(cbtoken) => match cbtoken {
                    CurlyBracketToken::Close => return Ok(result),
                    CurlyBracketToken::Open => bail!(format!("Unexpected {}", RCBRACKET)),
                },
                other => bail!(format!("Unexpected token {:?}", other)),
            }
        }
        Ok(result)
    }

    fn parse_statement(tokens: &[Token], ctx: &mut ParseContext) -> Result<StatementNode> {
        let statement_node = match tokens.get(ctx.cur).unwrap() {
            Token::Put => StatementNode::Put(Self::parse_put(tokens, ctx)?),
            Token::Variable(_) | Token::Head => {
                StatementNode::Assign(Self::parse_assign(tokens, ctx)?)
            }
            other => bail!(format!("Expected statement, found {:?}", other)),
        };
        let terminator = tokens.get(ctx.cur).unwrap_or(&Token::NewLine);
        if *terminator != Token::Semicolon && *terminator != Token::NewLine {
            bail!("statement must be terminated by \";\" or newline");
        }
        Ok(statement_node)
    }

    fn parse_while(tokens: &[Token], ctx: &mut ParseContext) -> Result<WhileNode> {
        ctx.cur += 1;
        if *Self::get_token_without_nl(tokens, ctx)? != Token::Bracket(BracketToken::Open) {
            bail!("\"while\" must be followed by \"(\"");
        }
        let Token::Variable(condition_token) = Self::get_token_without_nl(tokens, ctx)? else {
            bail!(format!(
                "expected variable, found {:?}",
                tokens.get(ctx.cur - 1).unwrap()
            ));
        };
        let condition = Self::parse_variable(condition_token);
        if *Self::get_token_without_nl(tokens, ctx)? != Token::Bracket(BracketToken::Close) {
            bail!("bracket mismatch, not enough \")\"");
        }
        if *Self::get_token_without_nl(tokens, ctx)? != Token::CurlyBracket(CurlyBracketToken::Open)
        {
            bail!("while must have loop content starts with \"{{\"");
        }
        let content = Self::parse_top_level_nodes(tokens, ctx)?;
        if *Self::get_token_without_nl(tokens, ctx)?
            != Token::CurlyBracket(CurlyBracketToken::Close)
        {
            bail!("bracket mismatch, not enough \"}}\"");
        }
        Ok(WhileNode { condition, content })
    }

    fn parse_put(tokens: &[Token], ctx: &mut ParseContext) -> Result<PutNode> {
        ctx.cur += 1;
        if *Self::get_token_without_nl(tokens, ctx)? != Token::Bracket(BracketToken::Open) {
            bail!("\"put\" must be followed by \"(\"");
        }
        let Token::Variable(char_token) = Self::get_token_without_nl(tokens, ctx)? else {
            bail!(format!(
                "expected variable, found {:?}",
                tokens.get(ctx.cur - 1).unwrap()
            ));
        };
        let character = Self::parse_variable(char_token);
        if *Self::get_token_without_nl(tokens, ctx)? != Token::Bracket(BracketToken::Close) {
            bail!("bracket mismatch, not enough \")\"");
        }
        Ok(PutNode { character })
    }

    fn parse_get(tokens: &[Token], ctx: &mut ParseContext) -> Result<GetNode> {
        ctx.cur += 1;
        if *Self::get_token_without_nl(tokens, ctx)? != Token::Bracket(BracketToken::Open) {
            bail!("\"get\" must be followed by \"(\"");
        }
        if *Self::get_token_without_nl(tokens, ctx)? != Token::Bracket(BracketToken::Close) {
            bail!("bracket mismatch, not enough \")\"");
        }
        Ok(GetNode)
    }

    fn parse_assign(tokens: &[Token], ctx: &mut ParseContext) -> Result<AssignNode> {
        let lvalue = match Self::get_token_without_nl(tokens, ctx)? {
            Token::Variable(var_token) => LValueNode::Variable(Self::parse_variable(var_token)),
            Token::Head => LValueNode::Head(HeadNode),
            ohter => bail!(format!("expected variable or head, found {:?}", ohter)),
        };
        let Token::Assign(assign_token) = Self::get_token_without_nl(tokens, ctx)? else {
            bail!(
                "expected assignment, found {:?}",
                tokens.get(ctx.cur - 1).unwrap()
            )
        };
        let rvalue = match Self::get_token_without_nl(tokens, ctx)? {
            Token::Number { value } => RValueNode::Number(NumberNode { value: *value }),
            Token::Get => RValueNode::Get(Self::parse_get(tokens, ctx)?),
            other => bail!(format!("expected number or get, found {:?}", other)),
        };
        let operand = AssignNodeOperand { lvalue, rvalue };
        Ok(match assign_token {
            AssginToken::Simple => AssignNode::Simple(operand),
            AssginToken::Add => AssignNode::Add(operand),
            AssginToken::Sub => AssignNode::Sub(operand),
        })
    }

    fn parse_variable(var_token: &VariableToken) -> VariableNode {
        match var_token {
            VariableToken::Static { index } => {
                VariableNode::Static(StaticVariableNode { index: *index })
            }
            VariableToken::Dynamic => VariableNode::Dynamic(DynamicVariableNode),
        }
    }

    fn get_token_without_nl<'a>(tokens: &'a [Token], ctx: &mut ParseContext) -> Result<&'a Token> {
        loop {
            let Some(token) = tokens.get(ctx.cur) else {
                bail!("Unexpected EOF")
            };
            match token {
                Token::NewLine => {
                    ctx.cur += 1;
                    continue;
                }
                other => {
                    ctx.cur += 1;
                    return Ok(other);
                }
            }
        }
    }

    pub fn new() -> Self {
        Parser
    }
}

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}
