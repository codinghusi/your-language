use crate::node::{Node, NodeEnum, NodeType};
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
        if let Some(token) = input.next() {
            if let Token::Identifier(identifier) = token.token {
                Ok(IdentifierNode {
                    name: identifier.to_string(),
                    span: input.span()
                })
            } else {
                Err(format!("Expected identifier, but got token '{}'", input.slice()))
            }
        } else {
            Err(format!("Expected identifier, but reached end of line"))
        }
    }

    fn span(&self) -> Span {
        self.span.clone()
    }
}