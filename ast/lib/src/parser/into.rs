use logos::Logos;

use crate::parser::buffer::ParseBuffer;

pub trait IntoParseBuffer<'source, Token>
where
    Token: Logos<'source> + Clone,
{
    fn parse_buffer(code: &'source str) -> ParseBuffer<'source, Token>;
}
