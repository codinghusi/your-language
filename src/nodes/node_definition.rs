use crate::node::{NodeEnum, NodeType};
use crate::nodes::identifier::IdentifierNode;
use crate::nodes::node_block::NodeBlockNode;
use logos::{Span, Lexer};
use crate::token::{Token, ParseBuffer, Result};
use node_derive::{NodeType, NodeEnum};
use std::iter::Peekable;

#[macro_use]
use crate::token;
use crate::parser::Parse;

#[derive(NodeType, Debug)]
pub struct NodeDefinitionNode {
    pub name: IdentifierNode,
    pub block: NodeBlockNode,
    pub span: Span
}

impl_parse!(NodeDefinitionNode, {
    (input) => {
        first!(keyword!(input, "node"))?;
        let name: IdentifierNode = input.parse()?;
        let block: NodeBlockNode = input.parse()?;
    },
    (span) => {
        Self {
            name,
            block,
            span
        }
    }
});
