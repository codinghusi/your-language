use logos::Span;
use crate::token::Token;
use crate::nodes::variable_declaration::VariableDeclarationNode;
use node_derive::NodeEnum;
use lib::parser::{
    parse::Parse
};
use lib::{ list, first };
use crate::{impl_parse, keyword, braced};
use serde::{Deserialize, Serialize};

#[derive(NodeEnum, Debug, Serialize, Deserialize)]
#[serde(tag = "_type")]
pub enum BlockItem {
    VariableDeclaration(VariableDeclarationNode),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct NodeBlockNode {
    pub items: Vec<BlockItem>,
    pub span: Span
}

impl_parse!(NodeBlockNode, {
    (input) => {
        braced!(input, curly {
            let items = first!(list!(input, BlockItem))?;
        });
    },
    (span) => {
        Self {
            items,
            span
        }
    }
});