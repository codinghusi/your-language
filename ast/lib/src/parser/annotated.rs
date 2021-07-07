use logos::{Span, Logos, Lexer};

pub struct AnnotatedLexer<'source, Token>
where Token: Logos<'source> {
    lexer: Lexer<'source, Token>
}

impl<'source, Token> Iterator for AnnotatedLexer<'source, Token>
where Token: Logos<'source> {
    type Item = (Token, Span, String);

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.lexer.next()?;
        let span = self.lexer.span();
        let slice = format!("{:?}", self.lexer.slice());
        Some((token, span, slice))
    }
}

pub trait AnnotatedLexi<'source, Token>
where Token: Logos<'source> {
    fn annotated(self) -> AnnotatedLexer<'source, Token>;
}

impl<'source, Token> AnnotatedLexi<'source, Token> for Lexer<'source, Token>
where Token: Logos<'source> {
    fn annotated(self) -> AnnotatedLexer<'source, Token> {
        AnnotatedLexer { lexer: self }
    }
}