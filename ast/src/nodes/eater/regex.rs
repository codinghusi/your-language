use crate::node_type::NodeType;
use logos::Span;
use crate::token::Token;
use node_derive::NodeType;
use crate::nodes::eater::EaterNode;
use lib::parser::parse::Parse;
use lib::{ first, token };
use crate::impl_parse;
use serde::{Deserialize, Serialize};

#[derive(NodeType, Debug, Serialize, Deserialize)]
pub struct RegexEater {
    value: String,
    span: Span
}

impl_parse!(RegexEater, {
    (input) => {
        let (str, _) = first!(token!(input, Token::Regex(str) => str))?;
    },
    (span) => {
        Self {
            value: str,
            span
        }
    }
});

impl<'source> EaterNode<'source, Token> for RegexEater { }