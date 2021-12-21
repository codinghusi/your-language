use lib::parser::parse::Parse;
use lib::{first, list};
use logos::Span;
use serde::{Deserialize, Serialize};

use crate::impl_parse;
use crate::nodes::eater::separator::SeparatedEater;
use crate::nodes::eater::Eater;
use crate::token::Token;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct ParserNode {
    first_eater: Eater,
    other_eaters: Vec<SeparatedEater>,
    span: Span,
}

impl_parse!(ParserNode, {
    (input) => {
        let first_eater: Eater = first!(input.parse())?;
        let other_eaters = list!(input, SeparatedEater)?;
    },
    (span) => {
        Self {
            first_eater,
            other_eaters,
            span
        }
    }
});
