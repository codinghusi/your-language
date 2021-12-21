use lib::parser::parse::Parse;
use lib::{first, token};
use logos::Span;
use serde::{Deserialize, Serialize};

use crate::impl_parse;
use crate::token::Token;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct IdentifierNode {
    pub value: String,
    pub span: Span,
}

impl_parse!(IdentifierNode, {
    (input) => {
        let (name, _) = first!(token!(input, Token::Identifier(name) => name, ["identifier"]))?;
    },

    (span) => {
        Self {
            value: name,
            span
        }
    }
});
