use crate::node_type::NodeType;
use logos::Span;
use crate::token::Token;
use node_derive::{NodeType, NodeEnum};
use crate::nodes::node_definition::NodeDefinitionNode;
use lib::parser::{
    parse::Parse
};
use lib::list;
use crate::impl_parse;
use serde::{Deserialize, Serialize};

#[derive(NodeEnum, Debug, Serialize, Deserialize)]
pub enum DocumentItem {
    // Import(ImportNode),
    Definition(NodeDefinitionNode)
}

#[derive(NodeType, Debug, Serialize, Deserialize)]
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
