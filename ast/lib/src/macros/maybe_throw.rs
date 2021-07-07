#[macro_export]
macro_rules! maybe_throw {
    ($result:expr) => {
        {
            let result = $result;
            match result {
                Err(err @ lib::parser::failure::ParseFailure::Poisoned(_)) => return Err(err),
                _ => result
            }
        }
    };
}
