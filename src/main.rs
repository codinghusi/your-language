
// #![feature(if_let_guard)]
// #![allow(dead_code)]
// #![allow(unused)]

#[macro_use]
mod token;
mod node;
mod nodes;
mod parser;

use crate::node::{NodeType, NodeEnum};

use crate::token::{Token, ParseBuffer, Result};
use crate::nodes::document::{DocumentNode};
use logos::Logos;
use crate::nodes::node_definition::NodeDefinitionNode;

fn main() -> Result<'static, ()> {
    let code = r"
        node Identifier {
            describe() => value: /[_a-zA-Z]\w*/;
        }
    ";
    //tokens: [Identifier("node"), Identifier("Identifier"), CurlyBrace(Open), Identifier("describe"), RoundedBrace(Open), RoundedBrace(Close), Assign, EaterName("value:"), Regex("/[_a-zA-Z]\\w*/"), Semicolon, CurlyBrace(Close)]

    let mut lexer = Token::lexer(code);
    // println!("tokens: {:?}", lexer.spanned().map(|(token, span)| token).collect::<Vec<_>>());
    let mut buffer = crate::parser::ParseBuffer::from(lexer);

    let node_definition: NodeDefinitionNode = buffer.parse()?;
    println!("{:#?}", node_definition);

    Ok(())
}




