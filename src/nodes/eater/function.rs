use crate::node::{NodeEnum, NodeType};
use logos::{Lexer, Span};
use crate::token::{Token, ParseBuffer, Result, Brace};
use node_derive::{NodeType, NodeEnum, node};
use crate::nodes::identifier::IdentifierNode;
use crate::nodes::eater::EaterNode;
use crate::parser::Parse;

#[derive(NodeType, Debug)]
pub struct FunctionEater {
    name: IdentifierNode,
    span: Span
}

impl_parse!(FunctionEater, {
    (input) => {
        let name: IdentifierNode = first!(input.parse())?;
        token!(input, Token::RoundedBrace(Brace::Open))?;
        token!(input, Token::RoundedBrace(Brace::Close))?;
    },
    (span) => {
        Self {
            name,
            span
        }
    }
});

impl<'source> EaterNode<'source, Token> for FunctionEater { }