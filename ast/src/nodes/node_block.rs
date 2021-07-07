use logos::Span;
use crate::token::Token;
use crate::nodes::variable_declaration::VariableDeclarationNode;
use crate::node::NodeType;
use node_derive::{NodeType, NodeEnum};
use lib::parser::{
    parse::Parse
};
use lib::{ braced, list };
use crate::impl_parse;


#[derive(NodeEnum, Debug)]
pub enum BlockItem {
    VariableDeclaration(VariableDeclarationNode),
}

#[derive(NodeType, Debug)]
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