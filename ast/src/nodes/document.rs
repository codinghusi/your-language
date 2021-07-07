use crate::node::NodeType;
use logos::Span;
use crate::token::Token;
use node_derive::{NodeType, NodeEnum};
use crate::nodes::node_definition::NodeDefinitionNode;
use lib::parser::{
    parse::Parse
};
use lib::list;
use crate::impl_parse;

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
