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

impl_parse!(IdentifierNode, {
    (input) => {
        let (name, _) = first!(token!(input, Token::Identifier(name) => name))?;
    },

    (span) => {
        Self {
            value: name,
            span
        }
    }
});