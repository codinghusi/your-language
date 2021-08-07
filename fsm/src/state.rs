use crate::edge::{Edge, EdgeType};
use std::vec::IntoIter;

pub enum InsertStatus {
    DidAlreadyExist,
    EOF,
    Added
}

#[derive(PartialEq)]
pub struct State<'a> {
    pub edge_index: u16,
    pub children: Vec<Edge<'a>>
}

impl<'a> State<'a> {
    pub fn new() -> Self {
        Self { children: vec![], edge_index: 0 }
    }

    pub fn add_path(&mut self, edges: &mut IntoIter<EdgeType<'a>>, mergeable: bool) -> InsertStatus {
        match edges.next() {
            Some(next) => {
                match (mergeable, self.children.iter_mut().find(|child| child.data.eq(&next))) {
                    (true, Some(ref mut existing_edge)) => match existing_edge.to_state.add_path(edges, true) {
                        InsertStatus::Added => {
                            self.edge_index += 1;
                            InsertStatus::Added
                        },
                        _ => InsertStatus::DidAlreadyExist
                    },
                    _ => {
                        let mut state = State::new();
                        state.add_path(edges, false);
                        let edge = Edge {
                            incr_value: self.edge_index,
                            data: next,
                            to_state: state
                        };
                        self.children.push(edge);
                        self.edge_index += 1;
                        InsertStatus::Added
                    }
                }
            },
            None => InsertStatus::EOF
        }
    }
}