use lib::parser::parse::Parse;
use lib::{first, list};
use logos::Span;
use node_derive::NodeEnum;
use serde::{Deserialize, Serialize};

use crate::nodes::variable_declaration::VariableDeclarationNode;
use crate::token::Token;
use crate::{braced, impl_parse, keyword};

#[derive(NodeEnum, Debug, Serialize, Deserialize)]
#[serde(tag = "_type")]
pub enum BlockItem {
    VariableDeclaration(VariableDeclarationNode),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct NodeBlockNode {
    pub items: Vec<BlockItem>,
    pub span: Span,
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
