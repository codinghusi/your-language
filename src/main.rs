
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

use crate::token::{Token, Result};
use crate::nodes::document::{DocumentNode};
use logos::Logos;

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
    let lexer = Token::lexer(code);
    let mut buffer = crate::parser::ParseBuffer::from(lexer);

    let document: DocumentNode = buffer.parse()?;
    println!("{:#?}", document);

    Ok(())
}




