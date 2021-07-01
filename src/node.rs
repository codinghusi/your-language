use crate::token::{Token, BaseLexer};
use logos::{Lexer, Span};
use std::iter::Peekable;

type ParserFn<T> = fn(&mut BaseLexer<'_>) -> Result<T, String>;

pub trait NodeType: Sized {
    fn get_type(&self) -> String;
}

pub trait Node: NodeType {
    fn parse(lexer: &mut BaseLexer<'_>) -> Result<Self, String>;
    fn span(&self) -> &Span;
}

pub trait NodeEnum {
    fn parse_any(lexer: &mut BaseLexer<'_>) -> Option<Self> where Self: Sized;
}

pub fn parse_one(lexer: &mut BaseLexer<'_>, node_parsers: &[&ParserFn<impl Node>]) -> Option<impl Node> {
    for node_parser in node_parsers {
        let result = node_parser(lexer);
        match result {
            Ok(node) => return Some(node),
            Err(_) => continue
        }
    }

    return None;
}
