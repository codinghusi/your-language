
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

fn main() -> Result<'static, ()> {
    let code = r"
        node Identifier {
            describe() => value: /[_a-zA-Z]\w*/;
        }
    ";

    let mut lexer = Token::lexer(code);
    let mut buffer = crate::parser::ParseBuffer::from(lexer);

    let span;
    spanned!(span, buffer, {
        let (name, token) = token!(buffer, Token::Identifier(identifier) => identifier)?;
        println!("name: {}", name);
    });

    // println!("span: {:?}, slice: {}", span, token.slice);

    Ok(())
}




