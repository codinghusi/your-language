
const VALID_SYMBOLS: &[&str] = &[
    "=>", "{", "}", "(", ")", ":", ";",
    "->", "-!>", "->>", "~>", "~!>", "~>>"
];

pub enum Token {
    Whitespace(Whitespace),
    Identifier(String),
    Symbol(String),
    Eater(Eater),
    EaterSeparator(EaterSeparator),
}

pub enum Whitespace {
    Ident(String),
    Dedent(String),
    Other(String),
}

pub enum Literal {
    Regex(String),
    String(String),
}

pub enum Eater {
    Node(String),
    Literal(Literal),
}

pub enum WhitespaceEater {
    Allowed,
    Required,
    Denied,
}

pub struct EaterSeparator {
    whitespace: WhitespaceEater,
    optional: bool,
}