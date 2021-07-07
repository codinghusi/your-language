#[macro_use]
use lib::token;

#[macro_export]
macro_rules! keyword {
    ($buffer:ident, $name:expr) => {
        {
            use lib::token;
            let name = $name.to_string();
            token!($buffer, crate::token::Token::Identifier(name))
        }
    };
}
