
#[macro_export]
macro_rules! braced {
    ($buffer:ident, ty $variant:ident, {$($body:tt)*}) => {
        use lib::token;
        use crate::token::{
            Token,
            Brace
        };
        token!($buffer, Token::$variant(Brace::Open), ["'{'"])?;
        $($body)*
        token!($buffer, Token::$variant(Brace::Close), ["'}'"])?;
    };

    ($buffer:ident, curly {$($body:tt)*}) => {
        braced!($buffer, ty CurlyBrace, { $($body)* });
    };

    ($buffer:ident, rounded {$($body:tt)*}) => {
        braced!($buffer, ty RoundedBrace, { $($body)* });
    };
}