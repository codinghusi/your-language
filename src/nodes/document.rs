use crate::node::{NodeEnum, NodeType};
use logos::{Lexer, Span};
use crate::token::{Token, ParseBuffer, Result};
use node_derive::{NodeType, NodeEnum};
use crate::nodes::node_definition::NodeDefinitionNode;
use std::borrow::Borrow;
use crate::parser::Parse;

#[derive(NodeEnum, Debug)]
pub enum DocumentItem {
    // Import(ImportNode),
    Definition(NodeDefinitionNode)
}

#[derive(NodeType, Debug)]
pub struct DocumentNode {
    pub items: Vec<DocumentItem>,
    span: Span
}

impl_parse!(DocumentNode, {
    (input) => {
        let items = list!(input, DocumentItem);
    },
    (span) => Self {
        items,
        span
    }
});
