use logos::Span;
use crate::token::Token;
use crate::nodes::variable_declaration::VariableDeclarationNode;
use crate::node_type::NodeType;
use node_derive::{NodeType, NodeEnum};
use lib::parser::{
    parse::Parse
};
use lib::{ braced, list };
use crate::impl_parse;
use serde::{Deserialize, Serialize};

#[derive(NodeEnum, Debug, Serialize, Deserialize)]
#[serde(tag = "_type")]
pub enum BlockItem {
    VariableDeclaration(VariableDeclarationNode),
}

#[derive(NodeType, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct NodeBlockNode {
    pub items: Vec<BlockItem>,
    pub span: Span
}

impl_parse!(NodeBlockNode, {
    (input) => {
        braced!(input, curly {
            let items = list!(input, BlockItem);
        });
    },
    (span) => {
        Self {
            items,
            span
        }
    }
});