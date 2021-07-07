use crate::node_type::NodeType;
use logos::Span;
use crate::token::Token;
use node_derive::NodeType;
use crate::nodes::eater::Eater;
use lib::parser::parse::Parse;
use crate::nodes::eater::separator::SeparatedEater;
use lib::{ first, list };
use crate::impl_parse;
use serde::{Deserialize, Serialize};

#[derive(NodeType, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct ParserNode {
    first_eater: Eater,
    other_eaters: Vec<SeparatedEater>,
    span: Span
}

impl_parse!(ParserNode, {
    (input) => {
        let first_eater: Eater = first!(input.parse())?;
        let other_eaters = list!(input, SeparatedEater);
    },
    (span) => {
        Self {
            first_eater,
            other_eaters,
            span
        }
    }
});