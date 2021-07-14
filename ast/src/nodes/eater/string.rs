use logos::Span;
use crate::token::Token;
use crate::nodes::eater::EaterNode;
use lib::parser::parse::Parse;
use lib::{ first, token };
use crate::impl_parse;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct StringEater {
    value: String,
    span: Span
}

impl_parse!(StringEater, {
    (input) => {
        let (str, _) = first!(token!(input, Token::String(str) => str))?;
    },
    (span) => {
        Self {
            value: str.clone(),
            span
        }
    }
});

impl<'source> EaterNode<'source, Token> for StringEater { }