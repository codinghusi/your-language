use lib::first;
use lib::parser::parse::Parse;
use logos::Span;
use serde::{Deserialize, Serialize};

use crate::impl_parse;
use crate::keyword;
use crate::nodes::identifier::IdentifierNode;
use crate::nodes::node_block::NodeBlockNode;
use crate::token;

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub struct CommentDefinitionNode {
    pub block: NodeBlockNode,
    pub span: Span,
}

impl_parse!(CommentDefinitionNode, {
    (input) => {
        first!(keyword!(input, "comments"))?;
        let block: NodeBlockNode = input.parse()?;
    },
    (span) => {
        Self {
            block,
            span
        }
    }
});
