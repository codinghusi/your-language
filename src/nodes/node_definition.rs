use crate::node::{Node, NodeEnum, NodeType};
use crate::nodes::identifier::IdentifierNode;
use crate::nodes::node_block::NodeBlockNode;
use logos::{Span, Lexer};
use crate::token::{Token, BaseLexer};
use node_derive::{NodeType, NodeEnum};
use crate::nodes::keyword::KeywordNode;
use std::iter::Peekable;

#[derive(NodeType)]
pub struct NodeDefinitionNode {
    name: IdentifierNode,
    block: NodeBlockNode,
    span: Span
}

impl Node for NodeDefinitionNode {
    fn parse(lexer: &mut BaseLexer) -> Result<Self, String> {
        let start = lexer.span().start;
        KeywordNode::parse(lexer)?.test_kw("node")?;
        let identifier = IdentifierNode::parse(lexer)?;
        let block = NodeBlockNode::parse(lexer)?;
        let end = lexer.span().end;
        let span = start..end;
        Ok(NodeDefinitionNode {
            name: identifier,
            block,
            span
        })
    }

    fn span(&self) -> &Span {
        &self.span
    }
}
