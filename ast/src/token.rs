use logos::Logos;
use crate::nodes::eater::separator::SeparationEater;
use lib::parser::buffer::ParseBuffer;
use lib::parser::into::IntoParseBuffer;

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

    #[regex(r#"/([^/]|\\\\|\\.)+/"#,  |lex| sliceit(&lex.slice().to_string(), 1, 1))]
    Regex(String),

    #[regex(r"[-~][!>]?>", |lex| SeparationEater::from_raw(lex.slice()))]
    Separator(SeparationEater),

    #[regex(r#""([^"])+""#, |lex| sliceit(&lex.slice().to_string(), 1, 1))]
    #[regex(r#"'([^'])+'"#, |lex| sliceit(&lex.slice().to_string(), 1, 1))]
    String(String),

    #[regex(r"([a-zA-Z_][a-zA-Z0-9_]*):", |lex| sliceit(&lex.slice().to_string(), 0, 1) )]
    EaterName(String),

    #[token(";")]
    Semicolon,

    #[regex(r"//[^\n]*\n", logos::skip)]
    #[regex(r"/\*([^*]|\*[^/])*\*/", logos::skip)]
    #[regex(r"[ \t\n]+", logos::skip)]
    #[error]
    Error,

    EOF
}

fn sliceit(slice: &str, left: usize, right: usize) -> String {
    slice[left..slice.len()-right].to_string()
}

impl<'source> IntoParseBuffer<'source, Token> for Token {
    fn parse_buffer(code: &'source str) -> ParseBuffer<'source, Token> {
        ParseBuffer::from(Self::lexer(&code.clone()))
    }
}






