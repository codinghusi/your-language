use std::fmt;
use crate::parser::{
    error::ParseError
};

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


