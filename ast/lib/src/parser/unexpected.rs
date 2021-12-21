use std::fmt;

use crate::parser::{err_values::ErrValues, token::ParseToken};

pub enum Got<Token>
where
    Token: Clone,
{
    EOF,
    Token(ParseToken<Token>),
}

pub struct Unexpected<Token>
where
    Token: Clone,
{
    pub expected: Vec<String>,
    pub got: Got<Token>,
}

impl<Token: Clone> fmt::Debug for Unexpected<Token> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let got_str = match &self.got {
            Got::EOF => "end of line",
            Got::Token(token) => &token.slice,
        };
        match self.expected.len() {
            0 => write!(f, "unexpected {}", got_str),
            1 => write!(f, "unexpected {}, expected: {}", got_str, self.expected[0]),
            _ => write!(
                f,
                "unexpected {}, expected one of: {}",
                got_str,
                self.expected.join(", ")
            ),
        }
    }
}
