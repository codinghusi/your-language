use crate::edge::{Edge, EdgeType, Capture};
use std::vec::IntoIter;
use std::iter;
use crate::{Path, PathItem};
use std::cell::{Ref, RefCell};
use std::borrow::BorrowMut;

#[derive(Debug)]
pub enum InsertStatus<'a> {
    DidAlreadyExist,
    Added(&'a mut Edge),
    EOF,
}

#[derive(PartialEq, Debug)]
pub struct State {
    pub edge_index: i16,
    pub edges: Vec<Edge>
}

impl State {
    pub fn new() -> Self {
        Self { edges: vec![], edge_index: 0 }
    }

    pub fn add_path(&mut self, items: IntoIter<PathItem>, mergeable: bool, target_value: i16) -> InsertStatus {
        match items.next() {
            Some(PathItem::Item(item)) => {
                // TODO: start here. Do refactor the following code!
                if let (true, Some(ref mut edge)) = (mergeable, self.edges.iter_mut().find(|child| child.data.eq(&item.to_edge_type()))) {
                    let diff = target_value - edge.value;
                    match edge.to_state.borrow_mut().add_path(items, true, diff) {
                        InsertStatus::EOF => InsertStatus::DidAlreadyExist,
                        status => status
                    }
                } else {
                    let mut state = State::new();
                    state.add_path(items, false, 0);
                    let edge = Edge {
                        value: target_value,
                        data: item.to_edge_type(),
                        to_state: RefCell::new(state),
                        capture: Capture::None
                    };
                    let borrow = &mut edge;
                    self.add_edge(edge);
                    InsertStatus::Added(borrow)
                }
            },
            Some(PathItem::SubItems(subitems)) => {
                if subitems.len() == 0 {
                    return self.add_path(items, mergeable, target_value)
                }
                let results: Vec<InsertStatus> = subitems.iter()
                    .map(|item| {
                        item.items.clone().into_iter().chain(item.items.clone().into_iter()).into_iter()
                    }) // FIXME: So bad cloning everything so often..
                    .map(|items| self.add_path(items, mergeable, target_value))
                    .collect();
                let failed = results.find(|status| match status {
                    InsertStatus::Added(_) => false,
                    _ => true
                });
                match failed {
                    Some(status) => status,
                    None => match results.into_iter().last() {
                        Some(last) => last,
                        None => unreachable!()
                    }
                }
            }
            None => InsertStatus::EOF
        }
    }

    pub fn add_edge(&self, edge: Edge) -> &Self {
        self.edges.push(edge);
        self
    }

    pub fn get_possible_states(&self) -> Vec<(char, &Box<State>)> {
        let mut result = vec![];
        for edge in self.edges.iter() {
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