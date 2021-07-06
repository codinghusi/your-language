use crate::node::NodeType;
use logos::Span;
use crate::token::Token;
use node_derive::NodeType;
use crate::nodes::eater::EaterNode;
use crate::parser::Parse;

#[derive(NodeType, Debug)]
pub struct StringEater {
    value: String,
    span: Span
}

impl_parse!(StringEater, {
    (input) => {
        let (str, _) = first!(token!(input, Token::String(str) => str))?;
    },
    (span) => {
        Self {
            value: str.clone(),
            span
        }
    }
});

impl<'source> EaterNode<'source, Token> for StringEater { }