use crate::node::{NodeEnum, NodeType};
use logos::{Lexer, Span};
use crate::token::{Token, ParseBuffer, Result};
use node_derive::{NodeType, NodeEnum};
use crate::nodes::eater::EaterNode;
use crate::parser::Parse;

#[derive(NodeType, Debug)]
pub struct RegexEater {
    value: String,
    span: Span
}

impl<'source> Parse<'source, Token> for RegexEater {
    fn parse(input: &mut ParseBuffer) -> Result<'source, Self> {
        let (str, token) = first!(token!(input, Token::Regex(str) => str))?;

        Ok(RegexEater {
            value: str,
            span: token.span()
        })
    }

    fn span(&self) -> Span {
        self.span.clone()
    }
}

impl<'source> EaterNode<'source, Token> for RegexEater { }