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