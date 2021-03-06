use lib::parser::parse::Parse;
use lib::{first, token};
use logos::Span;
use serde::{Deserialize, Serialize};

use crate::impl_parse;
use crate::nodes::eater::{EaterItem, EaterNode};
use crate::nodes::identifier::IdentifierNode;
use crate::token::Token;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct NamedEater {
    name: IdentifierNode,
    eater: EaterItem,
    span: Span,
}

impl_parse!(NamedEater, {
    (input) => {
        let (name, name_token) = first!(token!(input, Token::EaterName(name) => name, ["eater-name"]))?;
        let name_span = name_token.span().clone();

        let eater: EaterItem = input.parse()?;
    },
    (span) => {
        Self {
            name: IdentifierNode {
                value: name.clone(),
                span: name_span
            },
            span,
            eater,
        }
    }
});

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct UnnamedEater {
    eater: EaterItem,
    span: Span,
}

impl_parse!(UnnamedEater, {
    (input) => {
        let eater: EaterItem = first!(input.parse())?;
    },
    (span) => {
        Self {
            span,
            eater,
        }
    }
});

impl<'source> EaterNode<'source, Token> for NamedEater {}
