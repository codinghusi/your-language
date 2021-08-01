use logos::Span;
use crate::token::Token;
use node_derive::NodeEnum;
use super::definition::{
    CommentDefinitionNode,
    EntrypointDefinitionNode,
    NodeDefinitionNode,
    NodelessDefinitionNode,
    WhitespaceDefinitionNode
};
use lib::parser::{
    parse::Parse
};
use lib::{ list, first };
use crate::impl_parse;
use serde::{Deserialize, Serialize};

#[derive(NodeEnum, Debug, Serialize, Deserialize)]
#[serde(tag = "_type")]
pub enum DocumentItem {
    // Import(ImportNode),
    Node(NodeDefinitionNode),
    Nodeless(NodelessDefinitionNode),
    Comment(CommentDefinitionNode),
    Entrypoint(EntrypointDefinitionNode),
    Whitespace(WhitespaceDefinitionNode),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct DocumentNode {
    pub items: Vec<DocumentItem>,
    span: Span
}

impl_parse!(DocumentNode, {
    (input) => {
        let items = first!(list!(input, DocumentItem))?;
    },
    (span) => Self {
        items,
        span
    }
});
