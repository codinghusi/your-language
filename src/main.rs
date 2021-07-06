
// #![feature(if_let_guard)]
// #![allow(dead_code)]
// #![allow(unused)]

#[macro_use]
mod token;
mod node;
mod nodes;
mod parser;
mod utils;
mod annotated_lexer;

use crate::node::{NodeType, NodeEnum};

use crate::token::{Token, ParseBuffer, Result};
use crate::nodes::document::{DocumentNode};
use logos::Logos;
use crate::nodes::node_definition::NodeDefinitionNode;

fn main() -> Result<'static, ()> {
    let code = r#"
        node Ident {
            describe() => value: /[_a-zA-Z]\w*/;
        }

        node String {
            describe() => '"' -!> value: until() -!> '"';
        }
    "#;
    // get all tokens: println!("tokens: {:?}", lexer.spanned().map(|(token, span)| token).collect::<Vec<_>>());
    let mut lexer = Token::lexer(code);
    let mut buffer = crate::parser::ParseBuffer::from(lexer);

    let document: DocumentNode = buffer.parse()?;
    println!("{:#?}", document);

    Ok(())
}




