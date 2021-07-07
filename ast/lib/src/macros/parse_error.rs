#[macro_export]
macro_rules! parse_error {
    ($buffer:expr, unexpected) => {
        let token = $buffer.peek();
        Err(
            crate::token::ParseFailure::Poisoned(
                crate::token::ParseError::Unexpected {
                    expected: vec![stringify!($match).to_string()],
                    got: (*token).clone()
                }
            )
        )
    }
}