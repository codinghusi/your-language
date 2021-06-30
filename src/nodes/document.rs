use crate::node::{Node, NodeEnum, NodeType};
use logos::{Lexer, Span};
use crate::token::Token;
use node_derive::{NodeType, NodeEnum};
use crate::nodes::node_definition::NodeDefinitionNode;

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

impl Node for DocumentNode {
    fn parse(lexer: &mut Lexer<Token>) -> Result<Self, String> {
        // todo!("Implement a more generalized function for that common thing");
        let mut items = vec![];
        let start = lexer.span().start;
        let mut end = start;
        while let Some(item) = Document::parse_any(lexer) {
            items.push(item);
            end = lexer.span().start;
        }
        let span = start..end;
        Ok(DocumentNode {
            items,
            span
        })
    }

    fn span(&self) -> &Span {
        &self.span
    }
}