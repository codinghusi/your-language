use crate::node::{NodeEnum, NodeType};
use logos::{Lexer, Span};
use crate::token::{Token, ParseBuffer, Result};
use node_derive::{NodeType, NodeEnum};
use crate::parser::Parse;

#[derive(NodeType, Debug)]
pub struct IdentifierNode {
    pub value: String,
    pub span: Span
}


impl<'source> Parse<'source, Token> for IdentifierNode {
    fn parse(input: &mut ParseBuffer) -> Result<'source, Self> {
        let (name, token) = token!(input, Token::Identifier(name) => name)?;
        Ok(Self {
            value: name,
            span: token.span()
        })
    }

    fn span(&self) -> Span {
        self.span.clone()
    }
}