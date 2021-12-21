#[macro_export]
macro_rules! first {
    ($stuff:expr) => {
        match $stuff {
            Ok(token) => Ok(token),
            Err(lib::parser::error::ParseError::Poisoned(unexpected)) => {
                Err(lib::parser::error::ParseError::Peeked(unexpected))
            }
            result => result,
        }
    };
}
