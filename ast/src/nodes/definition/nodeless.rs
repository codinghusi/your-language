use crate::nodes::identifier::IdentifierNode;
use crate::nodes::node_block::NodeBlockNode;
use logos::Span;
use crate::token;
use lib::parser::parse::Parse;
use lib::{ first };
use crate::keyword;
use crate::impl_parse;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub struct NodelessDefinitionNode {
    pub name: IdentifierNode,
    pub block: NodeBlockNode,
    pub span: Span
}

impl_parse!(NodelessDefinitionNode, {
    (input) => {
        first!(keyword!(input, "nodeless"))?;
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
