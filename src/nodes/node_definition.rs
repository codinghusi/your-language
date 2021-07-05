use crate::node::{NodeEnum, NodeType};
use crate::nodes::identifier::IdentifierNode;
use crate::nodes::node_block::NodeBlockNode;
use logos::{Span, Lexer};
use crate::token::{Token, ParseBuffer, Result};
use node_derive::{NodeType, NodeEnum};
use std::iter::Peekable;

#[macro_use]
use crate::token;
use crate::parser::Parse;

#[derive(NodeType, Debug)]
pub struct NodeDefinitionNode {
    pub name: IdentifierNode,
    pub block: NodeBlockNode,
    pub span: Span
}

impl<'source> Parse<'source, Token> for NodeDefinitionNode {
    fn parse(input: &mut ParseBuffer) -> Result<'source, Self> {

        let span;
        spanned!(span, input, {
            keyword!(input, "node")?;
            let identifier: IdentifierNode = input.parse()?;
            let block: NodeBlockNode = input.parse()?;
        });

        Ok(Self {
            name: identifier,
            block,
            span
        })
    }

    fn span(&self) -> Span {
        self.span.clone()
    }
}
