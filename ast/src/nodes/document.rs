use lib::parser::parse::Parse;
use lib::{first, list};
use logos::Span;
use node_derive::NodeEnum;
use serde::{Deserialize, Serialize};

use crate::impl_parse;
use crate::token::Token;

use super::definition::{
    CommentDefinitionNode, EntrypointDefinitionNode, NodeDefinitionNode, NodelessDefinitionNode,
    WhitespaceDefinitionNode,
};

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
    span: Span,
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
