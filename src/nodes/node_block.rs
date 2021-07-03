use logos::{Lexer, Span};
use crate::token::{Token, Brace, ParseBuffer};
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
    fn parse(input: &mut ParseBuffer) -> Result<Self, String> {
        braced!(input, curly {
            while let Some(node) = BlockItem::parse_any(input) {
                unimplemented!();
            }
        });

        Ok(NodeBlockNode {
            span: 0..1
        })
    }

    fn span(&self) -> &Span {
        &self.span
    }
}