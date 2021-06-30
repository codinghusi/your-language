use crate::node::{Node, NodeEnum, NodeType};
use logos::{Lexer, Span};
use crate::token::Token;
use node_derive::{NodeType, NodeEnum};
use crate::nodes::eater::EaterNode;

#[derive(NodeType)]
pub struct StringEater {
    value: String,
    span: Span
}

impl Node for StringEater {
    fn parse(lexer: &mut Lexer<Token>) -> Result<Self, String> {
        if let Some(Token::String(str)) = lexer.next() {
            Ok(StringEater {
                value: str.clone(),
                span: lexer.span()
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