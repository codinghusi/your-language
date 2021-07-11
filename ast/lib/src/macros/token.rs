#[macro_export]
macro_rules! token {
    (util_poison: $buffer:ident, $expected:expr, ($token:ident) => $got:expr) => {
        {
            use lib::parser::{
                error::ParseError,
                unexpected::{
                    Unexpected,
                    Got
                }
            };
            let $token = $buffer.peek().unwrap();
            Err(
                ParseError::Poisoned(
                    Unexpected {
                        expected: $expected,
                        got: $got
                    }
                )
            )
        }
    };

    ($buffer:ident, $match:pat $(, $expected:expr)?) => {
        match token!($buffer, $match => () $(, $expected)?) {
            Ok(tuple) => Ok(tuple.1),
            Err(err) => Err(err)
        }
    };
    
    ($buffer:ident, $match:pat => $ret:expr $(, $expected:expr)?) => {
        token!($buffer, $match if true => $ret $(, $expected)?)
    };

    ($buffer:ident, $match:pat $( if $condition:expr )? => $ret:expr) => {
        token!($buffer, $match $( if $condition )? => $ret, vec![stringify!($match).to_string()])
    };

    ($buffer:ident, $match:pat $( if $condition:expr )? => $ret:expr, $expected:expr) => {
        {
            use lib::parser::token::ParseToken;
            let peek = $buffer.peek();
            if let Some(_) = peek {
                match peek {
                    Some(ParseToken { token: $match, .. }) $( if $condition )? => {
                        if let Some(token) = $buffer.next() {
                            if let $match = token.token.clone() {
                                Ok(($ret, token))
                            } else {
                                unreachable!();
                            }
                        } else {
                            unreachable!();
                        }
                    },
                    _ => token!(util_poison: $buffer, $expected, (token) => Got::Token((*token).clone()))
                }
            } else {
                token!(util_poison: $buffer, $expected, (token) => Got::EOF)
            }
        }
    };
}

