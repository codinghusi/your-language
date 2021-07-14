use logos::Span;
use crate::token::Token;
use lib::parser::parse::Parse;
use lib::{ first, token };
use crate::impl_parse;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
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