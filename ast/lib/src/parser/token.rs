use logos::Logos;

use crate::parser::span::Span;

#[derive(Clone)]
pub struct ParseToken<Token>
where
    Self: Sized,
    Token: Clone,
{
    pub token: Token,
    pub _span: Span,
    pub slice: String,
}

// TODO: implement From<T>
impl<'source, Token> From<(Token, Span, String)> for ParseToken<Token>
where
    Token: Clone + Logos<'source>,
{
    fn from(tuple: (Token, Span, String)) -> Self
    where
        Token: Logos<'source>,
    {
        let (token, _span, slice) = tuple;
        Self {
            token,
            _span,
            slice,
        }
    }
}

impl<Token: Clone> ParseToken<Token> {
    pub fn span(&self) -> &Span {
        &self._span
    }
}
