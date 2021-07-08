use std::fmt;
use crate::parser::{
    unexpected::Unexpected
};

pub enum ParseError<Token: Clone> {
    Peeked(Unexpected<Token>),
    Poisoned(Unexpected<Token>),
}

impl<Token: Clone> fmt::Debug for ParseError<Token> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Peeked(error) => write!(f, "Peeking failed: {:?}", error),
            Self::Poisoned(error) => write!(f, "Parsing failed: {:?}", error),
        }
    }
}


