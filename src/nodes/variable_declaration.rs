use crate::nodes::identifier::IdentifierNode;
use crate::node::{Node, NodeEnum, NodeType};
use logos::{Lexer, Span};
use crate::token::{Token, Brace, BaseLexer};
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
    fn parse(lexer: &mut BaseLexer) -> Result<Self, String> {
        let name = IdentifierNode::parse(lexer)?;

        // parses "() => "
        if let Some(Token::RoundedBrace(Brace::Open)) = lexer.next() { }
        else {
            Err(format!("Expected ("))?
        }

        if let Some(Token::RoundedBrace(Brace::Close)) = lexer.next() { }
        else {
            Err(format!("Expected )"))?
        }

        if let Some(Token::Assign) = lexer.next() { }
        else {
            Err(format!("Expected =>"))?
        }

        // parses the actual eater
        if let Some(eater) = Eater::parse_any(lexer) {
            let span = name.span().start..lexer.span().end;

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