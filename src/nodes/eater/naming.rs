use crate::nodes::eater::{EaterItem, EaterNode};
use crate::nodes::identifier::IdentifierNode;
use crate::nodes::eater::separator::SeparationEater;

use logos::{Lexer, Span};
use node_derive::{NodeEnum, NodeType};

use crate::node::{NodeEnum, NodeType};
use crate::token::{Token, ParseBuffer, Result};
use crate::parser::Parse;

#[derive(NodeType, Debug)]
pub struct NamedEater {
    name: IdentifierNode,
    eater: EaterItem,
    span: Span
}

impl_parse!(NamedEater, {
    (input) => {
        let (name, name_token) = first!(token!(input, Token::EaterName(name) => name))?;
        let name_span = name_token.span();

        let eater: EaterItem = input.parse()?;
    },
    (span) => {
        Self {
            name: IdentifierNode {
                value: name.clone(),
                span: name_token.span()
            },
            span,
            eater,
        }
    }
});

#[derive(NodeType, Debug)]
pub struct UnnamedEater {
    eater: EaterItem,
    span: Span
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

impl<'source> EaterNode<'source, Token> for NamedEater { }