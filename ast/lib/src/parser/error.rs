use std::fmt;
use crate::parser::{
    token::ParseToken
};

pub enum ParseError<Token>
    where Token: Clone {
    Unexpected {
        expected: Vec<String>,
        got: ParseToken<Token>
    },
    EOF {
        expected: Vec<String>
    },

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


