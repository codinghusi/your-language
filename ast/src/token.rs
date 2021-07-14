use logos::Logos;
use crate::nodes::eater::separator::SeparationEater;
use lib::parser::buffer::ParseBuffer;
use lib::parser::into::IntoParseBuffer;
use node_derive::TokenEnum;
use lib::parser::err_values::ErrValues;

#[derive(Clone, Debug)]
pub enum Brace {
    Open, Close
}

#[derive(Logos, TokenEnum, Clone, Debug)]
pub enum Token {
    #[name("identifier")]
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Identifier(String),

    // #[token("{"), |_| Brace::Open]
    // #[token("}"), |_| Brace::Close]
    #[values("{", "}")]
    #[regex("[{}]", |lex| if lex.slice().eq("{") {Brace::Open} else {Brace::Close})]
    CurlyBrace(Brace),
    
    // #[token("("), |_| Brace::Open]
    // #[token(")"), |_| Brace::Close]
    #[values("(", ")")]
    #[regex("[()]", |lex| if lex.slice().eq("(") {Brace::Open} else {Brace::Close})]
    RoundedBrace(Brace),

    #[token("=>")]
    Assign,

    #[name("regex")]
    #[regex(r#"/([^/]|\\.)+/"#,  |lex| sliceit(&lex.slice().to_string(), 1, 1))]
    Regex(String),
    
    #[values("->", "->>", "-!>", "~>", "~>>", "~!>")]
    #[regex(r"[-~][!>]?>", |lex| SeparationEater::from_raw(lex.slice()))]
    Separator(SeparationEater),
    
    
    #[name("regex")]
    #[regex(r#""([^"])+""#, |lex| sliceit(&lex.slice().to_string(), 1, 1))]
    #[regex(r#"'([^'])+'"#, |lex| sliceit(&lex.slice().to_string(), 1, 1))]
    String(String),

    #[name("eater name")]
    #[regex(r"([a-zA-Z_][a-zA-Z0-9_]*):", |lex| sliceit(&lex.slice().to_string(), 0, 1) )]
    EaterName(String),

    #[token(";")]
    Semicolon,

    #[regex(r"//[^\n]*\n", logos::skip)]
    #[regex(r"/\*([^*]|\*[^/])*\*/", logos::skip)]
    #[regex(r"[ \t\n]+", logos::skip)]
    #[error]
    Error,
}

fn sliceit(slice: &str, left: usize, right: usize) -> String {
    slice[left..slice.len()-right].to_string()
}

impl<'source> IntoParseBuffer<'source, Token> for Token {
    fn parse_buffer(code: &'source str) -> ParseBuffer<'source, Token> {
        ParseBuffer::from(Self::lexer(&code.clone()))
    }
}






