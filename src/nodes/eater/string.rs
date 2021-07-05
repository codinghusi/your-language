use crate::node::{NodeEnum, NodeType};
use logos::{Lexer, Span};
use crate::token::{Token, ParseBuffer, Result};
use node_derive::{NodeType, NodeEnum};
use crate::nodes::eater::EaterNode;
use crate::parser::Parse;

#[derive(NodeType)]
pub struct StringEater {
    value: String,
    span: Span
}

impl<'source> Parse<'source, Token> for StringEater {
    fn parse(input: &mut ParseBuffer) -> Result<'source, Self> {
        let (str, token) = token!(input, Token::String(str) => str)?;

        Ok(StringEater {
            value: str.clone(),
            span: token.span()
        })
    }

    fn span(&self) -> Span {
        self.span.clone()
    }
}

impl<'source> EaterNode<'source, Token> for StringEater { }