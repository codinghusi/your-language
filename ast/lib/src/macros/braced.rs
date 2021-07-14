
// FIXME: move into crate "ast"
#[macro_export]
macro_rules! braced {
    ($buffer:ident, ty $type:ident, {$($body:tt)*}) => {
        use lib::token;
        use crate::token::{
            Token,
            Brace
        };
        token!($buffer, Token::$type(Brace::Open))?;
        $($body)*
        token!($buffer, Token::$type(Brace::Close))?;
    };

    ($buffer:ident, curly {$($body:tt)*}) => {
        braced!($buffer, ty CurlyBrace, { $($body)* });
    };

    ($buffer:ident, rounded {$($body:tt)*}) => {
        braced!($buffer, ty RoundedBrace, { $($body)* });
    };
}