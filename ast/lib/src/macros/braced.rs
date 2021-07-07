
#[macro_export]
macro_rules! braced {
    ($buffer:ident, ty $type:ident, {$($body:tt)*}) => {
        use lib::token;
        token!($buffer, crate::token::Token::$type(crate::token::Brace::Open))?;
        $($body)*
        token!($buffer, crate::token::Token::$type(crate::token::Brace::Close))?;
    };

    ($buffer:ident, curly {$($body:tt)*}) => {
        braced!($buffer, ty CurlyBrace, { $($body)* });
    };

    ($buffer:ident, rounded {$($body:tt)*}) => {
        braced!($buffer, ty RoundedBrace, { $($body)* });
    };
}