use lib::parser::parse::Parse;
use lib::{first, token};
use logos::Span;
use serde::{Deserialize, Serialize};

use crate::impl_parse;
use crate::nodes::eater::EaterNode;
use crate::nodes::identifier::IdentifierNode;
use crate::token::{Brace, Token};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct FunctionEater {
    name: IdentifierNode,
    span: Span,
}

impl_parse!(FunctionEater, {
    (input) => {
        let name: IdentifierNode = first!(input.parse())?;
        token!(input, Token::RoundedBrace(Brace::Open), ["'('"])?;
        token!(input, Token::RoundedBrace(Brace::Close), ["')'"])?;
    },
    (span) => {
        Self {
            name,
            span
        }
    }
});

impl<'source> EaterNode<'source, Token> for FunctionEater {}
