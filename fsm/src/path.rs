use crate::EdgeType;
use std::str::Chars;

pub struct Path {
    pub items: Vec<PathItem>
}

impl Path {
    pub fn new() -> Self {
        Self { items: vec![] }
    }

    pub fn append<T>(&mut self, elements: T) -> Self
    where T: Iterator<Item=PathItem> + Sized {
        self.items.append(elements);
        self
    }

    pub fn add(&mut self, element: PathItem) -> Self {
        self.items.push(element);
        self
    }

    pub fn item(&mut self, element: Item) -> Self {
        self.add(PathItem::Item(element));
        self
    }

    pub fn char(&mut self, char: char) -> Self {
        self.item(Item::Char(char));
        self
    }

    pub fn jump(&mut self, position: u16) -> Self {
        self.item(Item::Jump(position));
        self
    }

    pub fn items(&mut self, elements: Vec<Item>) -> Self {
        self.append(elements.iter());
        self
    }

    pub fn oneOf(&mut self, chars: &str) -> Self {
        self.add(PathItem::OneOf(chars.chars()));
        self
    }

    pub fn subitems(&mut self, items: Vec<Item>) -> Self {
        self.add(PathItem::SubItems(items));
        self
    }


    pub fn string(&mut self, str: &str) -> Self {
        for c in str.chars() {
            self.char(c);
        }
        self
    }
}

impl From<Path> for Vec<EdgeType> {
    fn from(other: Path) -> Self {
        other.items.iter().map(Item::to_edge_type).collect()
    }
}

pub enum Item {
    Char(char),
    Jump(u16), // jumps N states backwards
}

// TODO: consider using Into<> / From<>
impl Item {
    pub fn to_edge_type(&self) -> EdgeType {
        match self {
            Self::Char(char) => EdgeType::Char(char),
            Self::Jump(n) => unimplemented!("jump not possible atm")
        }
    }
}

pub enum PathItem {
    SubItems(Vec<Path>),
    Item(Item)
}

impl PathItem {
    pub fn OneOf(chars: Chars) -> Self {
        Self::SubItems(chars.collect())
    }
}
