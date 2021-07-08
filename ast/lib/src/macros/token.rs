#[macro_export]
macro_rules! token {
    ($buffer:ident, $match:pat) => {
        match token!($buffer, $match => ()) {
            Ok(tuple) => Ok(tuple.1),
            Err(err) => Err(err)
        }
    };

    ($buffer:ident, $match:pat => $ret:expr) => {
        {
            use lib::parser::token::ParseToken;
            let peek = $buffer.peek();
            if let Some(_) = peek {
                if std::matches!(peek, Some(ParseToken { token: $match, .. })) {
                    if let Some(token) = $buffer.next() {
                        if let $match = token.token.clone() {
                            Ok(($ret, token))
                        } else {
                            unreachable!();
                        }
                    } else {
                        unreachable!();
                    }
                } else {
                    use lib::parser::{
                        error::ParseError,
                        unexpected::{
                            Unexpected,
                            Got
                        }
                    };
                    let token = $buffer.peek().unwrap();
                    Err(
                        ParseError::Poisoned(
                            Unexpected {
                                expected: vec![stringify!($match).to_string()],
                                got: Got::Token((*token).clone())
                            }
                        )
                    )
                }
            } else {
                use lib::parser::{
                    error::ParseError,
                    unexpected::{
                        Unexpected,
                        Got
                    }
                };
                Err(
                    lib::parser::error::ParseError::Poisoned(
                        Unexpected {
                            expected: vec![stringify!($variant).to_string()],
                            got: Got::EOF
                        }
                    )
                )
            }
        }
    };
}

