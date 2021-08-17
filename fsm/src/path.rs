use crate::EdgeType;
use std::str::Chars;
use std::mem;
use core::panicking::AssertKind::Match;

pub struct Path {
    pub items: Vec<PathItem>
}

impl Path {
    pub fn new() -> Self {
        Self { items: vec![] }
    }

    pub fn merge_items(&mut self, other_items: IntoIter<PathItem>) -> Self {

    }

    pub fn merge(&mut self, other: Path) -> Self {
        self.merge_items(other.items.into_iter())
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
        self.add(PathItem::Subpaths(items));
        self
    }


    pub fn string(&mut self, str: &str) -> Self {
        for c in str.chars() {
            self.char(c);
        }
        self
    }
}

impl From<Vec<PathItem>> for Path {
    fn from(items: Vec<PathItem>) -> Self {
        Path {
            items
        }
    }
}

trait Mergeable {
    fn merge(&mut left: Self, &mut right: Self) -> Self
    where Self: Sized;
}

impl Mergeable for Vec<PathItem> {
    /// Currently only merging prefix
    fn merge(&mut self, &mut other: Self) -> Self {
        let mut my_items = self.items.iter_mut();
        let mut other_items = other.into_iter();
        for other_item in other_items {
            match my_items.next() {
                Some(ref mut my_item) => {
                    fn matchit(left: &mut PathItem, right: PathItem) {
                        match (left, right) {

                            (left @ PathItem::Optional(my), right @ PathItem::Item(other)) |
                            (left @ PathItem::Item(my), right @ PathItem::Optional(other)) |
                            (left @ PathItem::Item(my), right @ PathItem::Item(other))
                            if !my.eq(other) => {
                                // Note: modify when adding suffix merge support
                                mem::replace(&mut left, PathItem::Subpaths(vec![
                                    left.into(),
                                    vec![right].extend(other_items).into()
                                ]));
                            },

                            (PathItem::Optional(my), PathItem::Optional(other))  => {
                                matchit(my, other)
                            },

                            (PathItem::Subpaths(paths), item) |
                            (item, PathItem::Subpaths(paths)) => {
                                for path in paths {

                                }
                            }

                            (PathItem::Subpaths(left_paths), PathItem::Subpaths(right_paths)) => {
                                for path in left_paths {

                                }
                            }


                        }
                    }
                    matchit(my_item, other_item)
                },
                None => {
                    // nothing to merge, just append the rest
                    self.items.extend(other_items);
                }
            }
        }
        self
    }
}

#[derive(PartialEq)]
pub enum Item {
    Char(char),
    Jump(u16), // jumps N states backwards
}

impl From<Item> for PathItem {
    fn from(item: Item) -> Self {
        Self::Item(item)
    }
}

impl From<PathItem> for Path {
    fn from(item: PathItem) -> Self {
        vec![item].into()
    }
}

impl From<Item> for Path {
    fn from(item: Item) -> Self {
        item.into::<PathItem>().into()
    }
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
    Subpaths(Vec<Path>),
    Optional(PathItem),
    Item(Item)
}

impl PathItem {
    pub fn OneOf(chars: Chars) -> Self {
        Self::Subpaths(chars.collect())
    }

    pub fn to_sub_items(self) -> Self {
        match &self {
            subitems @ Self::Subpaths(_) => subitems,
            item @ Self::Item(_) |
            item @ Self::Optional(_) => Self::Subpaths(vec![item].into()),
        }
    }

    pub fn difference(&mut self, path_item: &PathItem) -> MatchDifference {
        match (self, path_item) {
            (Self::Item(item), Self::Item(other))
                if item.eq(other) => MatchDifference::NoDiff,

            (Self::Optional(item), Self::Optional(other))
                if item.matches_item_mut(other) => MatchDifference::NoDiff,

            (item, Self::Subpaths(paths)) => {
                'loop: for path in paths {
                    let mut first = true;
                    for other in path.items {
                        match item.matches_item_mut(other) {
                            MatchDifference::NoDiff => (),
                            _ if first => continue 'loop,
                            diff => return diff
                        }
                        first = false;
                    }
                }
                MatchDifference::Diff {}
            },

            (Self::Subpaths(items), Self::Subpaths(others)) => {
                for item in items {

                }
            }

        }
    }
}

pub enum MatchDifference<'a> {
    NoDiff,
    Diff {
        left: &'a mut PathItem,
        right: &PathItem,
    },
}
