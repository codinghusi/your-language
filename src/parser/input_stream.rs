#![feature(if_let_guard)]

pub struct InputStream {
    position: usize,
    line: usize,
    column: usize,

    text: &str,

    currentNewline: Option<Newline>
}

enum Newline {
    Linux,
    MacOS,
    Windows
}

enum Item {
    Char(char),
    Newline(Newline)
}

impl InputStream {
    fn testFor(&self, str: &str) -> bool {
        self.text[self.position..str.len()].eq(str)
    }

    fn peek(&self) -> Option<Item> {
        self.peek_nth(0)
    }

    fn peek_nth(&self, offset: usize) -> Option<Item> {
        fn charAt(text: &str, position: usize) -> Option<char> {
            text.chars().nth(position)
        }

        let pos = self.position + offset;

        if let Some(char) = charAt(self.text, pos) {
            match char {
                '\n' => Some(Item::Newline(Newline::Linux)),
                '\r' if let Some('\n') = charAt(self.text, pos + 1) => Some(Item::Newline(Newline::Windows)),
                '\r' => Some(Item::Newline(Newline::MacOS)),
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
        if let Some(Item::Newline(newline)) = item {
            if let Newline::Windows = newline {
                self.position += 2;
            } else {
                self.position += 1;
            }
            self.column = 0;
            self.line += 1;
        }
        item
    }
}