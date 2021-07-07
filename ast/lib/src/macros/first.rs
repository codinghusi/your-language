
#[macro_export]
macro_rules! first {
    ($stuff:expr) => {
        {
            let result = $stuff;
            if let Ok(token) = result {
                Ok(token)
            } else if let Err(lib::parser::failure::ParseFailure::Poisoned(failure)) = result {
                Err(lib::parser::failure::ParseFailure::Peeked(failure))
            } else {
                result
            }
        }
    };
}