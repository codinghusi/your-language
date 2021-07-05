use crate::node::{NodeEnum, NodeType};
use logos::{Lexer, Span};
use crate::token::{Token, ParseBuffer, Result};
use node_derive::{NodeType, NodeEnum};
use crate::parser::Parse;

#[derive(NodeType)]
pub struct IdentifierNode {
    pub name: String,
    pub span: Span
}


impl<'source> Parse<'source, Token> for IdentifierNode {
    fn parse(input: &mut ParseBuffer) -> Result<'source, Self> {
        let name;
        let token = token!(input, Token::Identifier(name))?;
        Ok(IdentifierNode {
            name,
            span: token._span
        })
    }

    fn span(&self) -> Span {
        self.span.clone()
    }
}