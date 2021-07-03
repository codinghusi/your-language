
// #![feature(if_let_guard)]
#![allow(dead_code)]
#![allow(unused)]

#[macro_use]
mod token;
mod node;
mod nodes;
mod parser;

use crate::node::{Node, NodeType, NodeEnum};

use crate::token::{Token, ParseBuffer};
use crate::nodes::document::{DocumentNode};
use logos::Logos;

fn main() {
    let code = r"
        node Identifier {
            describe() => value: /[_a-zA-Z]\w*/;
        }
    ";

    let mut lexer = Token::lexer(code);
    let mut buffer = ParseBuffer::from(&mut lexer);

    let span;
    spanned!(span, buffer, {
        let identifier = token!(buffer, Identifier(capture)).unwrap();
    });

    println!("span: {:?}, slice: {}", span, identifier.slice);

    // let mut lex = Token::lexer(code);
    //
    // let parsed = DocumentNode::parse(&mut lex);
}




