use logos::{Logos, Lexer, SpannedIter, Span};
use crate::nodes::eater::separator::{SeparationEater, SeparatedEater};
use std::iter::Peekable;
use std::fmt;
use crate::parser;

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


pub type ParseBuffer<'source> = parser::ParseBuffer<'source, Token>;
pub type Result<'source, T> = parser::Result<'source, T, Token>;
pub type ParseToken = parser::ParserToken<Token>;
pub type ParseError= parser::ParseError<Token>;
pub type ParseFailure = parser::ParseFailure<Token>;

#[macro_export]
macro_rules! token {
    // results
    (result ok, $token:expr) => {
        Ok($token)
    };

    (result poisoned, $token:expr, $variant:expr) => {
        Err(
            crate::token::ParseFailure::Poisoned(
                crate::token::ParseError::Unexpected {
                    expected: vec![token!(str_variant, $variant)],
                    got: $token
                }
            )
        )
    };

    (result eof, $variant:expr) => {
        Err(
            crate::token::ParseFailure::Poisoned(
                crate::token::ParseError::EOF {
                    expected: vec![token!(str_variant, $variant)]
                }
            )
        )
    };

    // variants
    (variant, $ident:ident (capture)) => {
        Token::$ident(token)
    };

    (variant, $ident:ident) => {
        Token::$ident
    };

    (str_variant, $($variant:tt)+) => {
        stringify!($($variant)+).to_string()
    };

    // token based matchers
    (token $token:expr, $($variant:tt)+) => {
        {
            if let Some(token) = $token {
                if let token!(variant, $($variant)+) = token.token {
                    token!(result ok, token)
                } else {
                    token!(result poisoned, token, $($variant)+)
                }
            } else {
                token!(result eof, $($variant)+)
            }
        }
    };

    (try token $token:expr, $($variant:tt)+) => {
        {
            let result = token!(token $token, $($variant)+);
            if let Ok(token) = result {
                Ok(token)
            } else if let Err(ParseFailure::Posioned(failure)) = result {
                Err(ParseFailure::Peeked(failure))
            } else {
                token
            }
        }
    };

    // buffer based matchers
    ($buffer:expr, try $($variant:tt)+) => {
        let token = token!(try token $buffer.peek(), $($variant)+);
        if token.is_ok() {
            $buffer.next();
        }
        token
    };

    ($buffer:expr, $($variant:tt)+) => {
        {
            let token = token!(token $buffer.peek(), $($variant)+);
            if token.is_ok() {
                $buffer.next();
            }
            token
        }
    };
}

#[macro_export]
macro_rules! spanned {
    ($destination:ident, $buffer:expr, {$($body:tt)*}) => {
        let start = $buffer.peek_span().start;
        $($body)*
        let end = $buffer.peek_span().end;
        $destination = start..end;
    };
}

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