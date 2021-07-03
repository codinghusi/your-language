use crate::node::{Node, NodeEnum, NodeType};
use logos::{Lexer, Span};
use crate::token::{Token, ParseBuffer};
use node_derive::{NodeType, NodeEnum};
use crate::nodes::eater::EaterNode;

#[derive(NodeType)]
pub struct StringEater {
    value: String,
    span: Span
}

impl Node for StringEater {
    fn parse(input: &mut ParseBuffer) -> Result<Self, String> {
        if let Some(Token::String(str)) = input.next() {
            Ok(StringEater {
                value: str.clone(),
                span: input.span()
            })
        } else {
            Err(format!("Expected a string"))
        }
    }

    fn span(&self) -> &Span {
        &self.span
    }
}

impl EaterNode for StringEater { }