use crate::nodes::identifier::IdentifierNode;
use crate::node_type::NodeType;
use logos::Span;
use crate::token::Token;
use node_derive::NodeType;
use lib::parser::parse::Parse;
use crate::nodes::parser::ParserNode;
use lib::{ first, braced, token };
use crate::impl_parse;
use serde::{Deserialize, Serialize};

#[derive(NodeType, Debug, Serialize, Deserialize)]
pub struct VariableDeclarationNode {
    name: IdentifierNode,
    parser: ParserNode,
    span: Span
}

impl_parse!(VariableDeclarationNode, {
    (input) => {
        let name: IdentifierNode = first!(input.parse())?;

        // parses "() => "
        braced!(input, rounded {});
        token!(input, Token::Assign);

        // parses the actual eater
        let parser: ParserNode = input.parse()?;

        token!(input, Token::Semicolon)?;
    },
    (span) => {
        Self {
            name,
            span,
            parser
        }
    }
});