use crate::node::{Node, NodeEnum, NodeType};
use logos::{Lexer, Span};
use crate::token::{Token, BaseLexer};
use node_derive::{NodeType, NodeEnum, node};
use crate::nodes::identifier::IdentifierNode;
use crate::nodes::eater::EaterNode;

#[node(start = "Identifier")]
pub struct FunctionEater {
    name: IdentifierNode,
    span: Span
}

impl Node for FunctionEater {
    fn parse(lexer: &mut BaseLexer) -> Result<Self, String> {
        unimplemented!()
    }

    fn span(&self) -> &Span {
        &self.span
    }
}

impl EaterNode for FunctionEater { }