use crate::nodes::identifier::IdentifierNode;
use logos::Span;
use crate::token::Token;
use lib::parser::parse::Parse;
use crate::nodes::parser::ParserNode;
use lib::{ first, token };
use crate::{impl_parse, braced};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
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