use crate::node::{NodeEnum, NodeType};
use logos::{Lexer, Span};
use crate::token::{Token, ParseBuffer, Result};
use node_derive::{NodeType, NodeEnum, node};
use crate::nodes::identifier::IdentifierNode;
use crate::nodes::eater::EaterNode;
use crate::parser::Parse;

#[derive(NodeType)]
pub struct FunctionEater {
    name: IdentifierNode,
    span: Span
}

impl<'source> Parse<'source,  Token> for FunctionEater {
    fn parse(input: &mut ParseBuffer) -> Result<'source, Self> {
        unimplemented!()
    }

    fn span(&self) -> Span {
        self.span.clone()
    }
}

impl<'source> EaterNode<'source, Token> for FunctionEater { }