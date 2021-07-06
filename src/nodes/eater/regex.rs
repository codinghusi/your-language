use crate::node::NodeType;
use logos::Span;
use crate::token::Token;
use node_derive::NodeType;
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