use lib::parser::parse::Parse;
use lib::{first, token};
use logos::Span;
use serde::{Deserialize, Serialize};

use crate::impl_parse;
use crate::nodes::eater::EaterNode;
use crate::token::Token;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct StringEater {
    value: String,
    span: Span,
}

impl_parse!(StringEater, {
    (input) => {
        let (str, _) = first!(token!(input, Token::String(str) => str, ["string"]))?;
    },
    (span) => {
        Self {
            value: str.clone(),
            span
        }
    }
});

impl<'source> EaterNode<'source, Token> for StringEater {}
