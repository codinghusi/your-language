use logos::{Logos, Lexer};
use crate::nodes::eater::separator::{SeparationEater, SeparatedEater};
use std::iter::Peekable;

#[derive(Debug)]
pub enum Brace {
    Open, Close
}

#[derive(Logos)]
pub enum Token {
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Identifier(String),

    #[regex("[{}]", |lex| if lex.slice().eq("{") {Brace::Open} else {Brace::Close})]
    CurlyBrace(Brace),

    #[regex("[()]", |lex| if lex.slice().eq("(") {Brace::Open} else {Brace::Close})]
    RoundedBrace(Brace),

    #[token("=>")]
    Assign,

    #[regex(r"/([^\\/]|\\.)+/",  |lex| lex.slice().to_string())]
    Regex(String),

    #[regex(r"/[-~][!>]?>/", |lex| SeparationEater::fromRaw(lex.slice()))]
    Separator(SeparationEater),

    #[regex("/\"([^\"])+\"/", |lex| lex.slice().to_string())]
    String(String),

    #[regex("([a-zA-Z_][a-zA-Z0-9_]*):", |lex| lex.slice().to_string())]
    EaterName(String),

    #[token(";")]
    Semicolon,

    #[regex(r"[ \t\n]+", logos::skip)]
    #[error]
    Error
}

pub type BaseLexer<'a> = Lexer<'a, Token>;

// pub trait LexerHelper {
//     fn keyword(&mut self, name: &str) -> Result<&str, String>;
//     fn identifier(&mut self) -> Result<&str, String>;
// }

// impl LexerHelper for Lexer<Token> {
//     fn keyword(&mut self, name: &str) -> Result<&str, String> {
//         let token = self.next();
//         if let Some(Self::Identifier(identifier)) = token {
//             if name.eq(identifier) {
//                 Ok(identifier)
//             } else {
//                 Err(format!("Expected keyword '{}', but got '{}'", name, identifier))
//             }
//         } else {
//             Err(format!("Expected keyword '{}', but got token '{}'", name, token.slice()))
//         }
//     }
//
//     fn identifier(&mut self) -> Result<&str, String> {
//         let token = self.next();
//         if let Some(Self::Identifier(identifier)) = token {
//             Ok(identifier)
//         } else {
//             Err(format!("Expected identifier, but got token '{}'", token.slice()))
//         }
//     }
// }

// macro_rules! expect {
//     ($lex:ident, $token:ident) => {
//         let token = $lex.next();
//         if let Token::$token(val) = token {
//
//         }
//     }
// }