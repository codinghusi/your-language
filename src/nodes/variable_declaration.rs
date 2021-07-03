use crate::nodes::identifier::IdentifierNode;
use crate::node::{Node, NodeEnum, NodeType};
use logos::{Lexer, Span};
use crate::token::{Token, Brace, ParseBuffer};
use node_derive::{NodeType, NodeEnum};
use crate::nodes::eater::string::StringEater;
use crate::nodes::eater::Eater;

#[derive(NodeType)]
pub struct VariableDeclarationNode {
    name: IdentifierNode,
    eater: Eater,
    span: Span
}

impl Node for VariableDeclarationNode {
    fn parse(input: &mut ParseBuffer) -> Result<Self, String> {
        let name = IdentifierNode::parse(input)?;

        // parses "() => "
        if let Some(Token::RoundedBrace(Brace::Open)) = input.next() { }
        else {
            Err(format!("Expected ("))?
        }

        if let Some(Token::RoundedBrace(Brace::Close)) = input.next() { }
        else {
            Err(format!("Expected )"))?
        }

        if let Some(Token::Assign) = input.next() { }
        else {
            Err(format!("Expected =>"))?
        }

        // parses the actual eater
        if let Some(eater) = Eater::parse_any(input) {
            let span = name.span().start..input.span().end;

            Ok(Self {
                name,
                span,
                eater
            })
        }
        else {
            Err(format!("Parser expected"))
        }


    }

    fn span(&self) -> &Span {
        &self.span
    }
}