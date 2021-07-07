use crate::nodes::eater::Eater;
use lib::parser::parse::Parse;
use crate::token::Token;
use logos::Span;
use lib::{ first, token };
use crate::impl_parse;

#[derive(Clone, Debug)]
pub enum Whitespace {
    Optional,
    Required,
    NotAllowed
}

// FIXME: Names are to similar (SeparatedEater and SeperatorEater)
#[derive(Debug)]
pub struct SeparatedEater {
    separator_before: SeparationEater,
    eater: Eater,
    span: Span
}

impl_parse!(SeparatedEater, {
    (input) => {
        let (separator_before, _) = first!(token!(input, Token::Separator(separator) => separator))?;
        let eater: Eater = input.parse()?;
    },
    (span) => {
        Self {
            separator_before,
            eater,
            span
        }
    }
});

#[derive(Clone, Debug)]
pub struct SeparationEater {
    whitespace: Whitespace,
    require_following_eater: bool
}

impl SeparationEater {
    pub fn from_raw(raw: &str) -> SeparationEater {
        match raw {
            "->" => Self {
                whitespace: Whitespace::Optional,
                require_following_eater: true
            },

            "->>" => Self {
                whitespace: Whitespace::Required,
                require_following_eater: true
            },

            "-!>" => Self {
                whitespace: Whitespace::NotAllowed,
                require_following_eater: true
            },

            "~>" => Self {
                whitespace: Whitespace::Optional,
                require_following_eater: false
            },

            "~>>" => Self {
                whitespace: Whitespace::Required,
                require_following_eater: false
            },

            "~!>" => Self {
                whitespace: Whitespace::NotAllowed,
                require_following_eater: false
            },


            _ => panic!("{} is not a separator eater", raw)
        }
    }
}