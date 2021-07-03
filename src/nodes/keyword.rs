use crate::node::{Node, NodeEnum, NodeType};
use logos::{Lexer, Span};
use crate::token::{Token, ParseBuffer};
use node_derive::{NodeType, NodeEnum};

#[derive(NodeType)]
pub struct KeywordNode {
    name: String,
    span: Span,
}

impl KeywordNode {
    pub fn test_kw(&self, name: &str) -> Result<bool, String>{
        if self.name.eq(name) {
            Ok(true)
        } else {
            Err(format!("Expected keyword '{}', but got token '{}'", name, self.name))
        }
    }
}

impl Node for KeywordNode {
    fn parse(input: &mut ParseBuffer) -> Result<Self, String> {
        if let Some(token) = input.next() {
            if let Token::Identifier(identifier_name) = token {
                Ok(KeywordNode {
                    name: identifier_name.to_string(),
                    span: input.span()
                })
            } else {
                Err(format!("Expected identifier, but got token '{}'", input.slice()))
            }
        } else {
            Err(format!("Expected identifier, but reached end of line"))
        }
    }

    fn span(&self) -> &Span {
        &self.span
    }
}