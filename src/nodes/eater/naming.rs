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

impl<'source> Parse<'source, Token> for NamedEater {
    fn parse(input: &mut ParseBuffer) -> Result<'source, Self> {
        let (name, name_token) = first!(token!(input, Token::EaterName(name) => name))?;

        let eater: EaterItem = input.parse()?;
        let eater_span = eater.span();

        Ok(Self {
            name: IdentifierNode {
                value: name.clone(),
                span: name_token.span()
            },
            span: eater_span,
            eater,
        })
    }

    fn span(&self) -> Span {
        self.span.clone()
    }
}

#[derive(NodeType, Debug)]
pub struct UnnamedEater {
    eater: EaterItem,
    span: Span
}

impl<'source> Parse<'source, Token> for UnnamedEater {
    fn parse(input: &mut ParseBuffer) -> Result<'source, Self> {
        let eater: EaterItem = input.parse()?;
        Ok(Self {
            span: eater.span(),
            eater,
        })
    }

    fn span(&self) -> Span {
        self.span.clone()
    }
}

impl<'source> EaterNode<'source, Token> for NamedEater { }