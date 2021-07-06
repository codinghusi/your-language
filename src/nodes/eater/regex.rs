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

impl_parse!(RegexEater, {
    (input) => {
        let (str, _) = first!(token!(input, Token::Regex(str) => str))?;
    },
    (span) => {
        Self {
            value: str,
            span
        }
    }
});

impl<'source> EaterNode<'source, Token> for RegexEater { }