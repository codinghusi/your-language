use logos::{Lexer, Span};
use crate::token::{Token, Brace, ParseBuffer, Result};
use crate::nodes::variable_declaration::VariableDeclarationNode;
use crate::node::{NodeEnum, NodeType};
use node_derive::{NodeType, NodeEnum};
use crate::parser::{Parse};
use crate::nodes::identifier::IdentifierNode;

#[derive(NodeEnum)]
pub enum BlockItem {
    VariableDeclaration(VariableDeclarationNode),
}

#[derive(NodeType)]
pub struct NodeBlockNode {
    span: Span
}

impl<'source> Parse<'source, Token> for NodeBlockNode {
    fn parse(input: &mut ParseBuffer) -> Result<'source, Self> {
        braced!(input, curly {
            while let Ok(node) = BlockItem::parse(input) {
                unimplemented!();
            }
        });

        Ok(Self {
            span: 0..1
        })
    }

    fn span(&self) -> Span {
        self.span.clone()
    }
}