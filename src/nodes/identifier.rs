use crate::node::{Node, NodeEnum, NodeType};
use logos::{Lexer, Span};
use crate::token::Token;
use node_derive::{NodeType, NodeEnum};

#[derive(NodeType)]
pub struct IdentifierNode {
    pub name: String,
    pub span: Span
}


impl Node for IdentifierNode {
    fn parse(lexer: &mut Lexer<Token>) -> Result<Self, String> {
        if let Some(token) = lexer.next() {
            if let Token::Identifier(identifier) = token {
                Ok(IdentifierNode {
                    name: identifier.to_string(),
                    span: lexer.span()
                })
            } else {
                Err(format!("Expected identifier, but got token '{}'", lexer.slice()))
            }
        } else {
            Err(format!("Expected identifier, but reached end of line"))
        }
    }

    fn span(&self) -> &Span {
        &self.span
    }
}