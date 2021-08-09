use crate::edge::{Edge, EdgeType, Capture};
use std::vec::IntoIter;
use std::iter;

#[derive(Debug)]
pub enum InsertStatus {
    DidAlreadyExist,
    EOF,
    Added
}

#[derive(PartialEq, Debug)]
pub struct State {
    pub edge_index: u16,
    pub children: Vec<Edge>
}

impl State {
    pub fn new() -> Self {
        Self { children: vec![], edge_index: 0 }
    }

    fn add<T>(&mut self, path: T) -> InsertStatus
    where T: Into<Vec<EdgeType>> {
        let vec: Vec<EdgeType> = path.into();
        self.add_path(&mut vec.into_iter(), true)
    }

    pub fn add_path(&mut self, edges: &mut IntoIter<EdgeType>, mergeable: bool) -> InsertStatus {
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
                        let foo = if let EdgeType::Char(c) = &next { c.to_string() } else { "".to_string() };
                        let mut state = State::new();
                        state.add_path(edges, false);
                        let edge = Edge {
                            incr_value: self.edge_index,
                            data: next,
                            to_state: Box::new(state),
                            capture: Capture::None
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
    pub fn get_possible_states(&self) -> Vec<(char, &Box<State>)> {
        let mut result = vec![];
        for edge in self.children.iter() {
            match edge.data {
                EdgeType::Char(c) => result.extend([(c, &edge.to_state)].iter()),
                EdgeType::Jump => result.append(&mut edge.to_state.get_possible_states()),
                EdgeType::Start => panic!("Illegal Start-Edge between between states")
            }
        };
        result
    }

    pub fn combinations(&self) -> Vec<String> {
        let mut result = vec![];
        let possible_states = self.get_possible_states();
        for (c, state) in &possible_states {
            let combinations = state.combinations();
            if combinations.len() == 0 {
                result.push(c.to_string());
            } else {
                result.extend(combinations.iter().map(|suffix| format!("{}-{}", c, suffix)));
            }
        }
        result
    }

    pub fn parse(&self, input: &str) {
        let mut chars = input.chars();
        let mut current_state = self;
        while let Some(ch) = chars.next() {
            if let Some((_, state)) = current_state.get_possible_states().iter().find(|(c, _)| c.eq(&ch)) {
                current_state = state;
                println!("matched '{}'", ch);
            } else {
                panic!("couldn't match '{}'", ch);
                break;
            }
        }
    }
}