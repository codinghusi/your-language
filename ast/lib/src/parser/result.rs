use crate::parser::error::ParseError;

pub type Result<'source, Result, Token> = std::result::Result<Result, ParseError<Token>>;
