use crate::node::{Node, NodeEnum, NodeType};
use logos::{Lexer, Span};
use crate::token::{Token, ParseBuffer};
use node_derive::{NodeType, NodeEnum, node};
use crate::nodes::identifier::IdentifierNode;
use crate::nodes::eater::EaterNode;

#[derive(NodeType)]
pub struct FunctionEater {
    name: IdentifierNode,
    span: Span
}

impl Node for FunctionEater {
    fn parse(input: &mut ParseBuffer) -> Result<Self, String> {
        unimplemented!()
    }

    fn span(&self) -> &Span {
        &self.span
    }
}

impl EaterNode for FunctionEater { }