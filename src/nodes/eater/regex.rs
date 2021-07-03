use crate::node::{Node, NodeEnum, NodeType};
use logos::{Lexer, Span};
use crate::token::{Token, ParseBuffer};
use node_derive::{NodeType, NodeEnum};
use crate::nodes::eater::EaterNode;

#[derive(NodeType)]
pub struct RegexEater {
    value: String,
    span: Span
}

impl Node for RegexEater {
    fn parse(input: &mut ParseBuffer) -> Result<Self, String> {
        if let Some(Token::Regex(str)) = input.next() {
            Ok(RegexEater {
                value: str.clone(),
                span: input.span()
            })
        } else {
            Err(format!("Expected a regex"))
        }
    }

    fn span(&self) -> &Span {
        &self.span
    }
}

impl EaterNode for RegexEater { }