use crate::node::{NodeEnum, NodeType};
use logos::{Lexer, Span};
use crate::token::{Token, ParseBuffer, Result};
use node_derive::{NodeType, NodeEnum};
use crate::nodes::node_definition::NodeDefinitionNode;
use std::borrow::Borrow;
use crate::parser::Parse;

#[derive(NodeEnum)]
pub enum Document {
    // Import(ImportNode),
    Definition(NodeDefinitionNode)
}

#[derive(NodeType)]
pub struct DocumentNode {
    pub items: Vec<Document>,
    span: Span
}

impl<'source> Parse<'source, Token> for DocumentNode {
    fn parse(input: &mut ParseBuffer) -> Result<'source, Self> {
        // todo!("Implement a more generalized function for that common thing");
        let mut items = vec![];
        let start = input.span().end;
        let mut end = start;
        while let Ok(item) = Document::parse(input) {
            items.push(item);
            end = input.span().end;
        }
        let span = start..end;
        Ok(DocumentNode {
            items,
            span
        })
    }

    fn span(&self) -> Span {
        self.span.clone()
    }
}