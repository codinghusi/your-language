use crate::nodes::identifier::IdentifierNode;
use crate::node::{NodeEnum, NodeType};
use logos::{Lexer, Span};
use crate::token::{Token, Brace, ParseBuffer, Result};
use node_derive::{NodeType, NodeEnum};
use crate::nodes::eater::string::StringEater;
use crate::nodes::eater::Eater;
use crate::parser::Parse;

#[derive(NodeType)]
pub struct VariableDeclarationNode {
    name: IdentifierNode,
    eater: Eater,
    span: Span
}

impl<'source> Parse<'source, Token> for VariableDeclarationNode {
    fn parse(input: &mut ParseBuffer) -> Result<'source, Self> {
        let name: IdentifierNode = input.parse()?;

        // parses "() => "
        braced!(input, rounded {});
        token!(input, Token::Assign);

        // parses the actual eater
        let eater: Eater = input.parse()?;

        let span = name.span().start..input.span().end;

        Ok(Self {
            name,
            span,
            eater
        })
    }

    fn span(&self) -> Span {
        self.span.clone()
    }
}