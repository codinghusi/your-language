#[macro_export]
macro_rules! token {
    ($buffer:ident, $match:pat) => {
        {
            match token!($buffer, $match => ()) {
                Ok(tuple) => Ok(tuple.1),
                Err(err) => Err(err)
            }
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
                    let token = $buffer.peek().unwrap();
                    Err(
                        lib::parser::failure::ParseFailure::Poisoned(
                            lib::parser::error::ParseError::Unexpected {
                                expected: vec![stringify!($match).to_string()],
                                got: (*token).clone()
                            }
                        )
                    )
                }
            } else {
                Err(
                    lib::parser::failure::ParseFailure::Poisoned(
                        lib::parser::error::ParseError::EOF {
                            expected: vec![stringify!($variant).to_string()]
                        }
                    )
                )
            }
        }
    };
}

