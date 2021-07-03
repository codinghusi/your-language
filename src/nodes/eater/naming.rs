use crate::nodes::eater::{EaterItem, EaterNode};
use crate::nodes::identifier::IdentifierNode;
use crate::nodes::eater::separator::SeparationEater;

use logos::{Lexer, Span};
use node_derive::{NodeEnum, NodeType};

use crate::node::{Node, NodeEnum, NodeType};
use crate::token::{Token, ParseBuffer};

#[derive(NodeType)]
pub struct NamedEater {
    name: IdentifierNode,
    eater: EaterItem,
    span: Span
}

impl Node for NamedEater {
    fn parse(input: &mut ParseBuffer) -> Result<Self, String> {
        if let Some(Token::EaterName(name)) = input.next() {
            let namespan = input.span();
            if let Some(eater) = EaterItem::parse_any(input) {
                Ok(Self {
                    name: IdentifierNode {
                        name: name.clone(),
                        span: namespan
                    },
                    eater,
                    span: input.span()
                })
            } else {
                Err("Expected eater")?
            }

        } else {
            Err("Expected an eater name")?
        }
    }

    fn span(&self) -> &Span {
        &self.span
    }
}

#[derive(NodeType)]
pub struct UnnamedEater {
    eater: EaterItem,
    span: Span
}

impl Node for UnnamedEater {
    fn parse(lexer: &mut ParseBuffer) -> Result<Self, String> {
        if let Some(eater) = EaterItem::parse_any(lexer) {
            Ok(Self {
                eater,
                span: lexer.span()
            })
        } else {
            Err("Expected eater")?
        }
    }

    fn span(&self) -> &Span {
        &self.span
    }
}

impl EaterNode for NamedEater { }