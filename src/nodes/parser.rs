use crate::node::NodeType;
use logos::Span;
use crate::token::Token;
use node_derive::NodeType;
use crate::nodes::eater::Eater;
use crate::parser::Parse;
use crate::nodes::eater::separator::SeparatedEater;

#[derive(NodeType, Debug)]
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