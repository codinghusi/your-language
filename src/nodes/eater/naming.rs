use crate::nodes::eater::{EaterItem, EaterNode};
use crate::nodes::identifier::IdentifierNode;
use crate::nodes::eater::separator::SeparationEater;

use logos::{Lexer, Span};
use node_derive::{NodeEnum, NodeType};

use crate::node::{NodeEnum, NodeType};
use crate::token::{Token, ParseBuffer, Result};
use crate::parser::Parse;

#[derive(NodeType)]
pub struct NamedEater {
    name: IdentifierNode,
    eater: EaterItem,
    span: Span
}

impl<'source> Parse<'source, Token> for NamedEater {
    fn parse(input: &mut ParseBuffer) -> Result<'source, Self> {
        let name: String;
        let name_token = token!(input, Token::EaterName(name))?;

        let eater: EaterItem = input.parse()?;

        Ok(Self {
            name: IdentifierNode {
                name,
                span: name_token.span()
            },
            eater,
            span: eater.span()
        })
    }

    fn span(&self) -> Span {
        self.span.clone()
    }
}

#[derive(NodeType)]
pub struct UnnamedEater {
    eater: EaterItem,
    span: Span
}

impl<'source> Parse<'source, Token> for UnnamedEater {
    fn parse(input: &mut ParseBuffer) -> Result<'source, Self> {
        let eater: EaterItem = input.parse()?;
        Ok(Self {
            eater,
            span: eater.span()
        })
    }

    fn span(&self) -> Span {
        self.span.clone()
    }
}

impl<'source> EaterNode<'source, Token> for NamedEater { }