use logos::Logos;
use crate::nodes::eater::separator::SeparationEater;
use crate::parser;

#[derive(Clone, Debug)]
pub enum Brace {
    Open, Close
}

#[derive(Logos, Clone, Debug)]
pub enum Token {
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Identifier(String),

    #[regex("[{}]", |lex| if lex.slice().eq("{") {Brace::Open} else {Brace::Close})]
    CurlyBrace(Brace),

    #[regex("[()]", |lex| if lex.slice().eq("(") {Brace::Open} else {Brace::Close})]
    RoundedBrace(Brace),

    #[token("=>")]
    Assign,

    #[regex(r#"/([^/]|\\\\|\\.)+/"#,  |lex| lex.slice().to_string())]
    Regex(String),

    #[regex(r"[-~][!>]?>", |lex| SeparationEater::from_raw(lex.slice()))]
    Separator(SeparationEater),

    #[regex(r#""([^"])+""#, |lex| lex.slice().to_string())]
    #[regex(r#"'([^'])+'"#, |lex| lex.slice().to_string())]
    String(String),

    #[regex(r"([a-zA-Z_][a-zA-Z0-9_]*):", |lex| lex.slice().to_string())]
    EaterName(String),

    #[token(";")]
    Semicolon,

    #[regex(r"[ \t\n]+", logos::skip)]
    #[error]
    Error,
}


pub type ParseBuffer<'source> = parser::ParseBuffer<'source, Token>;
pub type Result<'source, T> = parser::Result<'source, T, Token>;
pub type ParseToken = parser::ParseToken<Token>;
pub type ParseError = parser::ParseError<Token>;
pub type ParseFailure = parser::ParseFailure<Token>;

#[macro_export]
macro_rules! maybe_throw {
    ($result:expr) => {
        {
            let result = $result;
            match result {
                Err(err @ crate::parser::ParseFailure::Poisoned(_)) => return Err(err),
                _ => result
            }
        }
    };
}

#[macro_export]
macro_rules! list {
    ($buffer:expr, $node:ty $(, $separator:pat)?) => {
        {
            let mut items = vec![];
            while let Ok(item) = maybe_throw!(<$node>::parse($buffer)) {
                items.push(item);
                $(
                    if token!($buffer, $separator).is_err() {
                        break;
                    }
                )?
            }
            items
        }
    };
}

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

#[macro_export]
macro_rules! delimited {
    ($buffer:expr, $node:ty, $separator:expr) => {
        {
            let mut leading = true;
            let mut items = vec![];
            while let Ok(item) = <$node>::parse($buffer) {
                items.push(item);
                if token!($buffer, $separator).is_err() {
                    leading = false;
                    break;
                }
            }
            if leading {

            }
            items
        }
    };
}

#[macro_export]
macro_rules! impl_parse {
    ($node:ty, {($input:ident) => {$($implementation:tt)*}, ($span:ident) => $return:expr}) => {
        impl<'source> Parse<'source, crate::token::Token> for $node {
            fn parse($input: &mut crate::token::ParseBuffer) -> crate::token::Result<'source, Self> {
                spanned!($input, {
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

#[macro_export]
macro_rules! spanned {
    ($buffer:expr, {body => {$($body:tt)*}, ($span:ident) => $return:expr}) => {
        {
            let start = $buffer.peek_span().start.clone();
            $($body)*
            let end = $buffer.span().end.clone();
            let $span = start..end;
            Ok($return)
        }
    };
}

#[macro_export]
macro_rules! identifier {
    ($buffer:ident, $capture:ident) => {
        token!($buffer, crate::token::Token::Identifier($capture))
    };
}

#[macro_export]
macro_rules! keyword {
    ($buffer:ident, $name:expr) => {
        {
            let name = $name.to_string();
            token!($buffer, crate::token::Token::Identifier(name))
        }
    };
}

#[macro_export]
macro_rules! token {
    ($buffer:ident, $match:pat) => {
        {
            match token!($buffer, $match => ()) {
                Ok(tuple) => Ok(tuple.1),
                Err(err) => Err(err)
            }
        }
    };

    ($buffer:ident, $match:pat => $ret:expr) => {
        {
            let peek = $buffer.peek();
            if let Some(_) = peek {
                if std::matches!(peek, Some(crate::parser::ParseToken { token: $match, .. })) {
                    if let Some(token) = $buffer.next() {
                        if let $match = token.token.clone() {
                            Ok(($ret, token))
                        } else {
                            unreachable!();
                        }
                    } else {
                        unreachable!();
                    }
                } else {
                    let token = $buffer.peek().unwrap();
                    Err(
                        crate::token::ParseFailure::Poisoned(
                            crate::token::ParseError::Unexpected {
                                expected: vec![stringify!($match).to_string()],
                                got: (*token).clone()
                            }
                        )
                    )
                }
            } else {
                Err(
                    crate::token::ParseFailure::Poisoned(
                        crate::token::ParseError::EOF {
                            expected: vec![stringify!($variant).to_string()]
                        }
                    )
                )
            }
        }
    };
}

#[macro_export]
macro_rules! braced {
    ($buffer:ident, ty $type:ident, {$($body:tt)*}) => {
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

#[macro_export]
macro_rules! first {
    ($stuff:expr) => {
        {
            let result = $stuff;
            if let Ok(token) = result {
                Ok(token)
            } else if let Err(crate::parser::ParseFailure::Poisoned(failure)) = result {
                Err(crate::parser::ParseFailure::Peeked(failure))
            } else {
                result
            }
        }
    };
}
