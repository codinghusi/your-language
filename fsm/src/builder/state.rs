use crate::path::{Path, Edge, CaptureTracker};
use std::ops::{RangeFrom, DerefMut, Deref};
use std::rc::Rc;
use std::cell::RefCell;
use std::slice::Iter;
use std::collections::HashSet;
use std::hash::Hasher;
use std::hash::Hash;
use std::borrow::{Borrow, BorrowMut};

pub enum MergeStatus {
    Added(Vec<StateRef>),
    Failed
}

#[derive(Clone, Debug)]
pub struct StateRef(Rc<RefCell<State>>);

impl StateRef {
    pub fn new(state: State) -> Self {
        Self(Rc::new(RefCell::new(state)))
    }
}

impl Borrow<State> for StateRef {
    fn borrow(&self) -> &State {
        self.0.as_ref().borrow().deref()
    }
}

impl BorrowMut<State> for StateRef {
    fn borrow_mut(&mut self) -> &mut State {
        self.0.as_ref().borrow_mut().deref_mut()
    }
}

impl Hash for StateRef {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.borrow() as &State).hash(state);
    }
}


// pub type StateRef = Rc<RefCell<State>>;

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::ptr::hash(self, state);
    }
}

#[derive(Debug)]
pub struct State {
    pub value: i16,
    pub nodes: Vec<StateRef>,
    pub transition: char
}

impl State {
    pub fn new_root() -> StateRef {
        // FIXME: it's dirty, because value and transition aren't used here
        StateRef::new(Self {
            value: 0,
            nodes: vec![],
            transition: '\0'
        })
    }

    pub fn add_node(&mut self, node: State) -> StateRef {
        let sref = StateRef::new(node);
        self.nodes.push(sref.clone());
        sref
    }

    pub fn get_node_by_transition(&mut self, transition: &char) -> Option<&StateRef> {
        self.nodes.iter().find(|node| (node.borrow() as &State).transition.eq(transition))
    }

    pub fn add_node_by_transition(&mut self, transition: &char, target_value: i16) -> (StateRef, i16) {
        println!("adding {}", transition);
        match self.get_node_by_transition(transition) {
            Some(state_ref) => (state_ref.clone(), target_value),
            None => (
                self.add_node(Self {
                    value: target_value,
                    nodes: vec![],
                    transition: *transition
                }),
                0
            )
        }

    }

    pub fn merge_path(root: StateRef, path: &Path, counter: &mut RangeFrom<i16>) -> MergeStatus {
        Self::merge_items(root, path.items.iter(), counter)
    }

    fn merge_states_with_path(states: &Vec<StateRef>, path: &Path, counter: &mut RangeFrom<i16>) -> MergeStatus {
        let mut end_states = vec![];
        let result = states.iter()
            .map(|state| Self::merge_path(state.clone(), path, counter))
            .any(|mut status| {
                match status {
                    MergeStatus::Added(ref mut states) => {
                        end_states.append(states);
                        true
                    },
                    MergeStatus::Failed => false
                }
            });

        if result {
            MergeStatus::Added(end_states)
        } else {
            MergeStatus::Failed
        }
    }

    pub fn merge_items(root: StateRef, items: Iter<Edge>, counter: &mut RangeFrom<i16>) -> MergeStatus {
        let mut capture_tracker = CaptureTracker::new();
        let mut target_value = counter.next().unwrap();
        let mut current_states = HashSet::new();
        current_states.insert(root);
        for item in items {
            let status = match item {
                Edge::Char(transition) => {
                    // FIXME: adjust the target_value!
                    let states = current_states.into_iter()
                        .map(|mut state| state.borrow_mut().add_node_by_transition(transition, target_value))
                        .map(|(state, _)| state) // follow up of fix me: the _ represents the remainder target_value
                        .collect();
                    MergeStatus::Added(states)
                },

                Edge::OneOf(possible_paths) => {
                    let mut all_ends = vec![];
                    let succeed = possible_paths
                        .iter()
                        .map(|path| Self::merge_states_with_path(&current_states, path, counter))
                        .all(|mut status| match status {
                            MergeStatus::Added(ref mut ends) => {
                                all_ends.append(ends);
                                true
                            },
                            MergeStatus::Failed => false
                        });
                    if succeed {
                        MergeStatus::Added(all_ends)
                    } else {
                        MergeStatus::Failed
                    }
                },

                Edge::Optional(optional_path) => {
                    match Self::merge_states_with_path(&current_states, optional_path, counter) {
                        MergeStatus::Added(mut ends) => {
                            ends.append(&mut current_states);
                            MergeStatus::Added(ends)
                        },
                        fail => fail
                    }
                },

                Edge::Capture(_) => {
                    unimplemented!()
                },

                Edge::Cycle(_) => {
                    unimplemented!()
                }
            };

            match status {
                MergeStatus::Added(states) => current_states = states,
                MergeStatus::Failed => panic!("aaah") // FIXME
            }
        }

        // Connecting last states with end state
        for mut state in &mut current_states {
            state.borrow_mut().add_node(Self {
                value: 0,
                nodes: vec![],
                transition: '\0'
            });
        }
        MergeStatus::Added(current_states)
    }

    pub fn all_combinations(&self) -> Vec<String> {
        let transition = if self.transition.eq(&'\0') {
            String::new()
        } else {
            self.transition.to_string()
        };
        if self.nodes.len() == 0 {
            return vec![transition];
        }
        let mut combinations = vec![];

        for node in &self.nodes {
            combinations.extend(node.borrow().all_combinations().iter().map(|combination| format!("{}{}", transition, combination)))
        }
        combinations
    }
}

