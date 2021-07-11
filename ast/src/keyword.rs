#[macro_use]
use lib::token;

#[macro_export]
macro_rules! keyword {
    ($buffer:ident, $name:expr) => {
        {
            use lib::token;
            token!($buffer, crate::token::Token::Identifier(kw) if kw.eq($name) => kw, vec![format!("\"{}\"", $name)])
        }
    };
}
