
// #![feature(if_let_guard)]
#![allow(dead_code)]
#![allow(unused)]

mod token;
mod node;
mod nodes;

use crate::node::{Node, NodeType, NodeEnum};

use crate::token::Token;
use crate::nodes::document::{DocumentNode};
use logos::Logos;

fn main() {
    let code = r"
        node Identifier {
            describe() => value: /[_a-zA-Z]\w*/;
        }
    ";

    let mut lex = Token::lexer(code);

    // DocumentNode::parse(lex);


    let parsed = DocumentNode::parse(&mut lex);
}




