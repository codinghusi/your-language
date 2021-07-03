use logos::{Span, SpannedIter, Lexer, Logos};
use std::fmt::{Display, Formatter, Debug};
use std::iter::Peekable;
use std::fmt;
use std::marker::PhantomData;

pub struct ParserToken<Token>  {
    pub token: Token,
    pub span: Span,
    pub slice: String
}

// TODO: implement From<T>
impl<Token> ParserToken<Token> {
    fn from<'source>(item: (Token, Span), slice: String) -> Self
    where Token: Logos<'source> {
        // TODO: Find a way to add slices to ParserToken
        ParserToken {
            token: item.0,
            span: item.1,
            slice
        }
    }
}

pub enum ParseError<Token> {
    Unexpected {
        expected: Vec<String>,
        got: ParserToken<Token>
    },
    EOF {
        expected: Vec<String>
    }
}

impl<Token> fmt::Debug for ParseError<Token> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Unexpected { expected, got: token } => {
                match expected.len() {
                    0 => write!(f, "unexpected {}", token.slice),
                    1 => write!(f, "unexpected {}, expected {}", token.slice, expected[0]),
                    _ => write!(f, "unexpected {}, expected one of {}", token.slice, expected.join(", ")),
                }
            },
            Self::EOF { expected } => {
                match expected.len() {
                    0 => write!(f, "unexpected end of line"),
                    1 => write!(f, "unexpected end of line, expected {}", expected[0]),
                    _ => write!(f, "unexpected end of line, expected one of {}", expected.join(", ")),
                }
            }
        }
    }
}

pub enum ParseFailure<Token> {
    Peeked(ParseError<Token>),
    Poisoned(ParseError<Token>)
}

impl<Token> fmt::Debug for ParseFailure<Token> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Peeked(error) => write!(f, "Peeking failed: {:?}", error),
            Self::Poisoned(error) => write!(f, "Parsing failed: {:?}", error),
        }
    }
}

pub type Result<'source, Result, Token: Logos<'source>> = std::result::Result<Result, ParseFailure<Token>>;

pub struct ParseBuffer<'source, Token>
where Token: Logos<'source> {
    pub lexer: Peekable<SpannedIter<'source, Token>>,
    pub last_span: Option<Span>
}

impl<'source, Token> ParseBuffer<'source, Token>
where Token: Logos<'source> {
    pub fn from(lexer: &mut Lexer<'source, Token>) -> Self {
        Self {
            lexer: lexer.spanned().peekable(),
            last_span: None
        }
    }

    pub fn parse<G>(&self) -> Result<G, Lexer<'source, Token>> {
        return G::parse(&self)
    }

    pub fn next(&mut self) -> Option<ParserToken<Token>> {
        let item = self.lexer.next()?;
        self.last_span = Some(item.1.clone());
        Some(ParserToken::from(item, self.slice()))
    }

    pub fn peek(&mut self) -> Option<ParserToken<Token>> {
        let item = self.lexer.peek()?;
        Some(ParserToken::from(item, self.slice()))
    }

    pub fn slice(&self) -> String {
        // FIXME: Actual find a way to get a slice
        return String::from("<not implemented>");
    }

    pub fn span(&mut self) -> Span {
        if let Some(span) = &self.last_span {
            span.clone()
        } else {
            0..0
        }
    }

    pub fn peek_span(&mut self) -> Span {
        if let Some(item) = self.lexer.peek() {
            item.1.clone()
        } else {
            let end = self.span().end;
            end..end
        }
    }
}

pub trait Parse<'source, Token>
where Token: Logos<'source>, Self: Sized {
    fn parse(input: &mut ParseBuffer<'source, Token>) -> Result<'source, Self, Token>;

    fn span(&self) -> Span;
}