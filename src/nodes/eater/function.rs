use crate::node::{Node, NodeEnum, NodeType};
use logos::{Lexer, Span};
use crate::token::Token;
use node_derive::{NodeType, NodeEnum};
use crate::nodes::identifier::IdentifierNode;
use crate::nodes::eater::EaterNode;

#[derive(NodeType)]
pub struct FunctionEater {
    name: IdentifierNode,
    span: Span
}

impl Node for FunctionEater {
    fn parse(lexer: &mut Lexer<Token>) -> Result<Self, String> {
        unimplemented!()
    }

    fn span(&self) -> &Span {
        &self.span
    }
}

impl EaterNode for FunctionEater { }