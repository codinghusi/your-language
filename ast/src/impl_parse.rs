
#[macro_export]
macro_rules! impl_parse {
    ($node:ty, {($input:ident) => {$($implementation:tt)*}, ($span:ident) => $return:expr}) => {
        impl<'source> Parse<'source, crate::token::Token> for $node {
            fn parse($input: &mut lib::parser::buffer::ParseBuffer<'source, crate::token::Token>) -> lib::parser::result::Result<'source, Self, crate::token::Token> {
                lib::spanned!($input, {
                    body => {
                        $($implementation)*
                    },
                    ($span) => $return
                })
            }

            fn span(&self) -> &Span {
                &self.span
            }
        }
    };
}