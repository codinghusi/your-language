use crate::node::{Node, NodeEnum, NodeType};
use crate::nodes::identifier::IdentifierNode;
use crate::nodes::node_block::NodeBlockNode;
use logos::{Span, Lexer};
use crate::token::{Token, ParseBuffer};
use node_derive::{NodeType, NodeEnum};
use crate::nodes::keyword::KeywordNode;
use std::iter::Peekable;

#[macro_use]
use crate::token;

#[derive(NodeType)]
pub struct NodeDefinitionNode {
    name: IdentifierNode,
    block: NodeBlockNode,
    span: Span
}

impl Node for NodeDefinitionNode {
    fn parse(input: &mut ParseBuffer) -> Result<Self, String> {
        // let span = parse!(lexer, {
        //     keyword!(lexer: "node");
        //     let identifier = @identifier();
        //     let block = @node(NodeBlockNode);
        // })?;

        let span;
        spanned!(span, input, {
            let identifier = token!(input, Identifier(capture))?;
            let block: NodeBlockNode = input.parse()?;
        });


        Ok(NodeDefinitionNode {
            name: identifier,
            block,
            span
        })


        // let start = lexer.span().start;
        // KeywordNode::parse(lexer)?.test_kw("node")?;
        // let identifier = IdentifierNode::parse(lexer)?;
        // let block = NodeBlockNode::parse(lexer)?;
        // let end = lexer.span().end;
        // let span = start..end;
        // Ok(NodeDefinitionNode {
        //     name: identifier,
        //     block,
        //     span
        // })
    }

    fn span(&self) -> &Span {
        &self.span
    }
}
