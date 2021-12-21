use logos::{Lexer, Logos};
use std::fmt::Debug;
use std::iter::Peekable;
use std::marker::PhantomData;
use std::vec::IntoIter;

use crate::maybe_unwrap;
use crate::parser::span::Span;
use crate::parser::{annotated::AnnotatedLexi, parse::Parse, result::Result, token::ParseToken};

pub struct ParseBuffer<'source, Token>
where
    Token: Logos<'source> + Clone,
{
    pub lexer: Peekable<IntoIter<ParseToken<Token>>>,
    pub last_span: Option<Span>,
    lifetime_stuff: PhantomData<&'source ()>,
}

impl<'source, Token> ParseBuffer<'source, Token>
where
    Token: Logos<'source> + Clone + Debug,
{
    pub fn from(lexer: Lexer<'source, Token>) -> Self {
        Self {
            lexer: lexer
                .annotated()
                .map(|(token, span, slice)| ParseToken::from((token, span, slice)))
                .collect::<Vec<_>>()
                .into_iter()
                .peekable(),
            last_span: None,
            lifetime_stuff: PhantomData,
        }
    }

    pub fn parse<G>(&mut self) -> Result<'source, G, Token>
    where
        G: Parse<'source, Token>,
    {
        return G::parse(self);
    }

    pub fn next(&mut self) -> Option<ParseToken<Token>> {
        let item = self.lexer.next();
        self.last_span = maybe_unwrap!(&item, Some(token) => token.span().clone());
        item
    }

    pub fn peek(&mut self) -> Option<&ParseToken<Token>> {
        self.lexer.peek()
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
            item.span().clone()
        } else {
            let end = self.span().end;
            end..end
        }
    }
}
