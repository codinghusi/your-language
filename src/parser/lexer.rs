use std::iter::Peekable;
use std::vec::IntoIter;
use std::{fs, io};
use peekmore::{
    PeekMore,
    PeekMoreIterator
};

use crate::parser::token::{
    Token, Whitespace
};

use crate::parser::input_stream::{
    InputStream
};

type Result<T> = std::result::Result<T, String>;
type Code = PeekMoreIterator<Peekable<IntoIter<char>>>;

pub struct Lexer {
    code: Code
}


impl Lexer {
    fn from_text(text: &str) -> Self {
        Lexer {
            code: text
                .chars()
                .collect::<Vec<_>>()
                .into_iter()
                .peekable()
                .peekmore(),
        }
    }

    fn from_file(file_path: &str) -> io::Result<Self> {
        Ok(Self::from_text(&fs::read_to_string(file_path)?))
    }

    // ----- parsing utils

    // TODO: Whitespace::Ident and Dedent not implemented
    fn eat_whitespace(&mut self) -> Option<Token> {
        let mut whitespaces = String::new();

        loop {
            match self.code.peek() {
                Some(c) if c.is_whitespace() => whitespaces.push(self.code.next().unwrap()),
                _ => break
            }
        }

        if whitespaces.len() > 0 {
            return Some(Token::Whitespace(Whitespace::Other(whitespaces)));
        }
        None
    }

    fn check_char(&self, char: char) -> bool {
        if let Some(compare) = self.code.peek() {
            if char == *compare {
                return true;
            }
        }
        false
    }

    fn check_str(&self, token_str: &str) -> bool {
        let chars = token_str.as_bytes();
        for c in chars.iter() {
            // check for eof
            if let None = self.code.peek() {
                self.code.reset_cursor();
                return false;
            }

            // revert if not working
            if !self.check_char(*c as char) {
                self.code.reset_cursor();
                return false;
            }

            // go further
            self.code.advance_cursor();
        }
        self.code.reset_cursor();
        return true;
    }

    fn eat_str(&self, token_str: &str) -> bool {
        if self.check_str(token_str) {
            self.code.nth(token_str.len());
            return true;
        }
        false
    }

    fn eat_until(&self, token_str: &str) -> String{
        let mut result = String::new();
        loop {
            if !self.eat_str(token_str) {
                result.push(self.code.next().unwrap());
                continue;
            }
            return result;
        }
    }

    fn eat_until_any(&self, strs: [&str]) -> String{
        let mut result = String::new();
        loop {
            if !self.eat_str(token_str) {
                result.push(self.code.next().unwrap());
                continue;
            }
            return result;
        }
    }

    // FIXME: not working
    // fn eat_while<F: Fn(&Lexer) -> bool>(&mut self, predicate: F) -> Result<String> {
    //     let mut result = String::new();
    //     loop {
    //         match self.code.peek() {
    //             Some(_) if predicate() => result.push(self.code.next().unwrap()),
    //             None => return Err("Unxepected reached end of file".to_string()),
    //             _ => break,
    //         }
    //     }
    //     Ok(result)
    // }
    
    fn eat_newline(&mut self) -> bool {
        self.eat_str("\n") || self.eat_str("\r\n") || self.eat_str("\r")
    }

    fn eat_comment(&mut self) -> bool {
        if self.eat_str("//") {
            self.eat_until()
            return true;
        }
        false
    }
}


impl Iterator for Lexer {
    type Item = Result<Token>;

    fn next(&mut self) -> Option<Self::Item> {

        let eaters = [
            Self::eat_whitespace,

        ];

        for eater in eaters.iter() {
            if let Some(token) = eater(&mut self) {
                return Some(Ok(token));
            }
        }

        None
    }

}