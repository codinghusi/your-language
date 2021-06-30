use crate::token::Token;
use logos::{Lexer, Span};

type ParserFn<T> = fn(&mut Lexer<Token>) -> Result<T, String>;

pub trait NodeType: Sized {
    fn get_type(&self) -> String;
}

pub trait Node: NodeType {
    fn parse(lexer: &mut Lexer<Token>) -> Result<Self, String>;
    fn span(&self) -> &Span;
}

pub trait NodeEnum {
    fn parse_any(lexer: &mut Lexer<Token>) -> Option<Self> where Self: Sized;
}

pub fn parse_one(lexer: &mut Lexer<Token>, node_parsers: &[&ParserFn<impl Node>]) -> Option<impl Node> {
    for node_parser in node_parsers {
        let result = node_parser(lexer);
        match result {
            Ok(node) => return Some(node),
            Err(_) => continue
        }
    }

    return None;
}
