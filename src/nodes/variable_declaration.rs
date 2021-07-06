use crate::nodes::identifier::IdentifierNode;
use crate::node::NodeType;
use logos::Span;
use crate::token::Token;
use node_derive::NodeType;
use crate::parser::Parse;
use crate::nodes::parser::ParserNode;

#[derive(NodeType, Debug)]
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