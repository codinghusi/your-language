
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
    Ident(Vec<SingleWhitespace>),
    Dedent(Vec<SingleWhitespace>),
    Other(Vec<SingleWhitespace>),
}

impl ToString for Whitespace {
    fn to_string(&self) -> String {
        fn whitespacesToString(whitespaces: &Vec<SingleWhitespace>) -> String {
            let mut result = String::new();
            for whitespace in whitespaces.iter() {
                result.push(whitespace.to_string());
            }
            result
        }

        match self {
            Self::Ident(ws) => whitespacesToString(ws),
            Self::Dedent(ws) => whitespacesToString(ws),
            Self::Other(ws) => whitespacesToString(ws),
        }
    }
}

pub enum SingleWhitespace {
    Newline(Newline),
    Space,
    Tab
}

impl SingleWhitespace {
    fn to_string(&self) -> char {
        match self {
            Self::Newline(_) => '\n',
            Self::Space => ' ',
            Self::Tab => '\t'
        }
    }
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

pub enum Newline {
    Linux,
    MacOS,
    Windows
}

pub enum Item {
    Char(char),
    Whitespace(SingleWhitespace)
}

