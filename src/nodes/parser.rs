use crate::node::{Node, NodeEnum, NodeType};
use logos::{Lexer, Span};
use crate::token::Token;
use node_derive::{NodeType, NodeEnum};

#[derive(NodeType)]
pub struct ParserNode {
    value: String,
    span: Span
}

impl Node for ParserNode {
    fn parse(lexer: &mut Lexer<Token>) -> Result<Self, String> {
        let eaters = vec![];
        while let Some(eater) = Eater::parse_any(lexer) {

        }
    }

    fn span(&self) -> &Span {
        &self.span
    }
}