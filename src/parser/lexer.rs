

use crate::parser::token::{
    Token, Whitespace
};

use crate::parser::input_stream::InputStream;

use crate::parser::token::{
    Item, SingleWhitespace
};

type Result<T> = std::result::Result<T, String>;
type Code = InputStream;

pub struct Lexer {
    code: Code
}


impl Lexer {
    // ----- parsing utils

    // TODO: Whitespace::Ident and Dedent not implemented
    fn eat_whitespace(&mut self) -> Option<Token> {
        let mut whitespaces = Vec::new();

        loop {
            match self.code.peek() {
                Some(item) if let Item::Whitespace(whitespace) = item => whitespaces.push(whitespace),
                _ => break
            }
        }

        if whitespaces.len() > 0 {
            return Some(Token::Whitespace(Whitespace::Other(whitespaces)));
        }
        None
    }

    fn check_char(&self, char: char) -> bool {
        if let Some(item) = self.code.peek() {
            if let Item::Char(compare) = item {
                if char == compare {
                    return true;
                }
            }
        }
        false
    }

    fn eat_str(&self, token_str: &str) -> bool {
        if self.code.testFor(token_str) {
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

    // fn eat_until_any(&self, strs: &[&String]) -> String{
    //     let mut result = String::new();
    //     loop {
    //         if !self.eat_str(token_str) {
    //             result.push(self.code.next().unwrap());
    //             continue;
    //         }
    //         return result;
    //     }
    // }

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