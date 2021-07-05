use logos::{Span, SpannedIter, Lexer, Logos};
use std::fmt::{Display, Formatter, Debug};
use std::iter::Peekable;
use std::fmt;
use std::slice::Iter;
use std::vec::IntoIter;
use std::marker::PhantomData;

#[derive(Clone)]
pub struct ParseToken<Token>
where Self: Sized, Token: Clone {
    pub token: Token,
    pub _span: Span,
    pub slice: String
}

// TODO: implement From<T>
impl<Token> ParseToken<Token>
where Token: Clone {
    fn from<'source>(item: (Token, Span), slice: String) -> Self
    where Token: Logos<'source> {
        // TODO: Find a way to add slices to ParserToken
        ParseToken {
            token: item.0,
            _span: item.1,
            slice
        }
    }

    pub fn span(&self) -> Span {
        self._span.clone()
    }
}

pub enum ParseError<Token>
where Token: Clone {
    Unexpected {
        expected: Vec<String>,
        got: ParseToken<Token>
    },
    EOF {
        expected: Vec<String>
    }
}

impl<Token: Clone> fmt::Debug for ParseError<Token> {
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

pub enum ParseFailure<Token: Clone> {
    Peeked(ParseError<Token>),
    Poisoned(ParseError<Token>),
    EnumCheck
}

impl<Token: Clone> fmt::Debug for ParseFailure<Token> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Peeked(error) => write!(f, "Peeking failed: {:?}", error),
            Self::Poisoned(error) => write!(f, "Parsing failed: {:?}", error),
            // TODO: implement to say what tokens were expected
            Self::EnumCheck => write!(f, "Tried to parse <not implemented>, but couldn't"),
        }
    }
}

pub type Result<'source, Result, Token> = std::result::Result<Result, ParseFailure<Token>>;

pub struct ParseBuffer<'source, Token>
where Token: Logos<'source> + Clone {
    pub lexer: Peekable<IntoIter<ParseToken<Token>>>,
    pub last_span: Option<Span>,
    next_token: Option<(Token, Span)>,
    lifetime_stuff: PhantomData<&'source ()>,
}

impl<'source, Token> ParseBuffer<'source, Token>
where Token: Logos<'source> + Clone {
    pub fn from(lexer: Lexer<'source, Token>) -> Self {
        Self {
            // FIXME: implement <not implemented>
            lexer: lexer.spanned().map(|item| ParseToken::from(item, "<not implemented>".to_string())).collect::<Vec<_>>().into_iter().peekable(),
            last_span: None,
            next_token: None,
            lifetime_stuff: PhantomData
        }
    }

    pub fn parse<G>(&mut self) -> Result<'source, G, Token>
    where G: Parse<'source, Token> {
        return G::parse(self)
    }

    pub fn next(&mut self) -> Option<ParseToken<Token>> {
        self.lexer.next()
    }

    pub fn peek(&mut self) -> Option<&ParseToken<Token>> {
        self.lexer.peek()
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
            item.span()
        } else {
            let end = self.span().end;
            end..end
        }
    }
}

pub trait Parse<'source, Token>
where Token: Logos<'source> + Clone, Self: Sized {
    fn parse(input: &mut ParseBuffer<'source, Token>) -> Result<'source, Self, Token>;

    fn span(&self) -> Span;
}