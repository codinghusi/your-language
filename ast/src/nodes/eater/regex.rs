use lib::parser::parse::Parse;
use lib::{first, token};
use logos::Span;
use serde::{Deserialize, Serialize};

use crate::impl_parse;
use crate::nodes::eater::EaterNode;
use crate::token::Token;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct RegexEater {
    value: String,
    span: Span,
}

impl_parse!(RegexEater, {
    (input) => {
        let (str, _) = first!(token!(input, Token::Regex(str) => str, ["regex"]))?;
    },
    (span) => {
        Self {
            value: str,
            span
        }
    }
});

impl<'source> EaterNode<'source, Token> for RegexEater {}
