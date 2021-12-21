use logos::Logos;
use std::fmt::Debug;

use crate::parser::{buffer::ParseBuffer, result::Result, span::Span};

pub trait Parse<'source, Token>
where
    Token: Logos<'source> + Clone,
    Self: Sized + Debug,
{
    fn parse(input: &mut ParseBuffer<'source, Token>) -> Result<'source, Self, Token>;

    fn span(&self) -> &Span;
}
