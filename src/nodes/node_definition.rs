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

#[derive(NodeType)]
pub struct NodeDefinitionNode {
    name: IdentifierNode,
    block: NodeBlockNode,
    span: Span
}

impl<'source> Parse<'source, Token> for NodeDefinitionNode {
    fn parse(input: &mut ParseBuffer) -> Result<'source, Self> {

        let span;
        spanned!(span, input, {
            let identifier;
            token!(input, Token::Identifier(identifier))?;
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

    fn span(&self) -> Span {
        self.span.clone()
    }
}
