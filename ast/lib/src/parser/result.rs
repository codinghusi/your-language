use crate::parser::{
    failure::ParseFailure
};

pub type Result<'source, Result, Token> = std::result::Result<Result, ParseFailure<Token>>;