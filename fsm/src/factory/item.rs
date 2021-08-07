use std::slice::Iter;
use std::vec::IntoIter;

pub struct Item<'a> {
    edge: EdgeItem<'a>,
    children: Vec<EdgeItem<'a>>
}

pub enum InsertStatus {
    DidAlreadyExist,
    EOF,
    Added
}

impl<'a> Item<'a> {
    pub fn new(edge: EdgeItem<'a>) -> Self<'a> {
        Self {
            edge,
            children: vec![]
        }
    }

    pub fn add_path(&mut self, items: IntoIter<Item<'a>>, mergeable: bool) -> InsertStatus {
        match items.next() {
            Some(next) => {
                match (mergable, self.children.iter().find(|child| child.eq(next))) {
                    (true, Some(ref existant)) => match existant.add_path(items, true) {
                        InsertStatus::Added => InsertStatus::Added,
                        _ => InsertStatus::DidAlreadyExist
                    },
                    _ => {
                        next.add_path(items, false);
                        self.children.push(next);
                        InsertStatus::Added
                    }
                }
            },
            None => InsertStatus::EOF
        }
    }
}