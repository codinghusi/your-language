use crate::parser::token::{
    Whitespace, Item, SingleWhitespace, Token, Newline
};
use std::{fs, io};



pub struct InputStream {
    position: usize,
    line: usize,
    column: usize,

    text: String,

    currentNewline: Option<Newline>
}

impl InputStream {
    pub fn from_text(text: &str) -> Self {
        InputStream {
            position: 0,
            line: 0,
            column: 0,
            text: String::new(),
            currentNewline: None
        }
    }

    pub fn from_file(file_path: &str) -> io::Result<Self> {
        Ok(Self::from_text(&fs::read_to_string(file_path)?))
    }

    pub fn testFor(&self, str: &str) -> bool {
        self.text[self.position..self.position + str.len()].eq(str)
    }

    pub fn peek(&self) -> Option<Item> {
        self.peek_nth(0)
    }

    pub fn peek_nth(&self, offset: usize) -> Option<Item> {
        fn charAt(text: &str, position: usize) -> Option<char> {
            text.chars().nth(position)
        }

        let pos = self.position + offset;

        if let Some(char) = charAt(&self.text, pos) {
            match char {
                '\n' => Some(Item::Whitespace(SingleWhitespace::Newline(Newline::Linux))),
                '\r' if let Some('\n') = charAt(&self.text, pos + 1) => Some(Item::Whitespace(SingleWhitespace::Newline(Newline::Windows))),
                '\r' => Some(Item::Whitespace(SingleWhitespace::Newline(Newline::MacOS))),
                '\t' => Some(Item::Whitespace(SingleWhitespace::Tab)),
                ' ' => Some(Item::Whitespace(SingleWhitespace::Space)),
                _ => Some(Item::Char(char))
            }
        } else {
            None
        }
    }
}

impl Iterator for InputStream {
    type Item = Item;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.peek();
        if let Some(Item::Whitespace(SingleWhitespace::Newline(newline))) = item {
            if let Newline::Windows = newline {
                self.position += 2;
            } else {
                self.position += 1;
            }
            self.column = 0;
            self.line += 1;
        } else {
            self.position += 1;
            self.column += 1;
        }
        item
    }
}