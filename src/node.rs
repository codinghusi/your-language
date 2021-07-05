use crate::token::{Token, ParseBuffer};
use logos::{Lexer, Span};
use std::iter::Peekable;
use crate::parser::{Result};
//
// type ParserFn<T> = fn(&mut ParseBuffer) -> Result<T, String>;

pub trait NodeType: Sized {
    fn get_type(&self) -> String;
}

pub trait NodeEnum<Token>
where Self: Sized, Token: Clone {
    fn parse_any<'source>(lexer: &mut ParseBuffer) -> Result<'source, Self, Token>;
}
//
// pub fn parse_one(lexer: &mut ParseBuffer, node_parsers: &[&ParserFn<impl Node>]) -> Option<impl Node> {
//     for node_parser in node_parsers {
//         let result = node_parser(lexer);
//         match result {
//             Ok(node) => return Some(node),
//             Err(_) => continue
//         }
//     }
//
//     return None;
// }
