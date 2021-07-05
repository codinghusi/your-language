use logos::{Lexer, Span};
use crate::token::{Token, Brace, ParseBuffer, Result};
use crate::nodes::variable_declaration::VariableDeclarationNode;
use crate::node::{NodeEnum, NodeType};
use node_derive::{NodeType, NodeEnum};
use crate::parser::{Parse};
use crate::nodes::identifier::IdentifierNode;

#[derive(NodeEnum, Debug)]
pub enum BlockItem {
    VariableDeclaration(VariableDeclarationNode),
}

#[derive(NodeType, Debug)]
pub struct NodeBlockNode {
    pub items: Vec<BlockItem>,
    pub span: Span
}

impl<'source> Parse<'source, Token> for NodeBlockNode {
    fn parse(input: &mut ParseBuffer) -> Result<'source, Self> {
        let span;
        spanned!(span, input, {
            braced!(input, curly {
                let items = list!(input, BlockItem);
            });
        });

        Ok(Self {
            items,
            span
        })
    }

    fn span(&self) -> Span {
        self.span.clone()
    }
}