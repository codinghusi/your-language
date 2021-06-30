use logos::{Lexer, Span};
use crate::token::{Token, Brace};
use crate::nodes::variable_declaration::VariableDeclarationNode;
use crate::node::{Node, NodeEnum, NodeType};
use node_derive::{NodeType, NodeEnum};

#[derive(NodeEnum)]
pub enum BlockItem {
    VariableDeclaration(VariableDeclarationNode)
}

#[derive(NodeType)]
pub struct NodeBlockNode {
    span: Span
}

impl Node for NodeBlockNode {
    fn parse(lexer: &mut Lexer<Token>) -> Result<Self, String> {
        if let Some(Token::CurlyBrace(Brace::Open)) = lexer.next() { }
        else {
            Err(format!("Expected {{ for starting a block"))?
        }

        while let Some(node) = BlockItem::parse_any(lexer) {
            unimplemented!();
        }

        if let Some(Token::CurlyBrace(Brace::Close)) = lexer.next() { }
        else {
            Err(format!("Expected }} for ending a block"))?
        }

        Ok(NodeBlockNode {
            span: 0..1
        })
    }

    fn span(&self) -> &Span {
        &self.span
    }
}