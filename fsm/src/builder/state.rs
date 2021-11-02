use crate::path::{Path, Edge, CaptureTracker};
use std::ops::{RangeFrom, DerefMut, Deref};
use std::rc::Rc;
use std::cell::RefCell;
use std::slice::Iter;
use std::collections::HashSet;
use std::hash::Hasher;
use std::hash::Hash;
use crate::builder::id_gen::IdGen;
use std::hint::unreachable_unchecked;
use crate::builder::tracker::Tracker;

pub enum MergeStatus {
    Added(HashSet<StateRef>),
    Failed
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StateRef(Rc<RefCell<State>>);

impl StateRef {
    pub fn new(state: State) -> Self {
        Self(Rc::new(RefCell::new(state)))
    }
}

impl Deref for StateRef {
    type Target = RefCell<State>;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl Hash for StateRef {
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        self.0.borrow().hash(state)
    }
}


impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::ptr::hash(&self, state);
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}

impl Eq for State {}

#[derive(Debug)]
pub struct State {
    pub value: usize,
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

    pub fn get_node_by_transition(&self, transition: &char) -> Option<&StateRef> {
        self.nodes.iter().find(|node| node.borrow().transition.eq(transition))
    }

    pub fn add_node_by_transition(&mut self, transition: &char, tracker: &Tracker) -> (StateRef, usize) {
        match self.get_node_by_transition(transition) {
            Some(state_ref) => (state_ref.clone(), tracker.current_target_value()),
            None => (
                self.add_node(Self {
                    value: tracker.current_target_value(),
                    nodes: vec![],
                    transition: *transition
                }),
                0
            )
        }
    }

    pub fn merge_path(root: StateRef, path: &Path, tracker: &mut Tracker) -> MergeStatus {
        Self::merge_items(root, path.items.iter(), tracker)
    }

    fn merge_states_with_path(states: &HashSet<StateRef>, path: &Path, tracker: &mut Tracker) -> MergeStatus {
        let mut failed = false;
        let result: Result<Vec<_>, _> = states
            .iter()
            .map(|state| Self::merge_path(state.clone(), path, &mut tracker.clone()))
            .map(|status| {
                match status {
                    MergeStatus::Added(mut states) => Ok(states.into_iter()),
                    failed => Err(failed)
                }
            })
            .collect();

        match result {
            Ok(states) => MergeStatus::Added(states.into_iter().flatten().collect()),
            Err(failed) => failed
        }
    }

    pub fn merge_items(root: StateRef, items: Iter<Edge>, tracker: &mut Tracker) -> MergeStatus {
        let mut current_states = HashSet::new();
        current_states.insert(root);

        // path items will be iterated,
        // nested items will be handled recursively
        for item in items {
            let status = match item {
                // given are n states (open ends of the fsm paths)

                // appends the char to all n states
                Edge::Char(transition) => {
                    // FIXME: adjust the target_value!

                    let states = current_states.into_iter()
                        .map(|mut state| state.borrow_mut().add_node_by_transition(transition, &tracker))
                        .map(|(state, _)| state) // follow up of fix me: the _ represents the remainder target_value
                        .collect();
                    MergeStatus::Added(states)
                },

                // splits all n states to n*m states, appends each 'n' all 'm' possible paths
                Edge::OneOf(possible_paths) => {
                    let mut all_ends = HashSet::new();
                    let succeed = possible_paths
                        .iter()
                        .map(|path| Self::merge_states_with_path(&current_states, path, &mut tracker.clone()))
                        .all(|mut status| match status {
                            MergeStatus::Added(mut ends) => {
                                all_ends.extend(ends.into_iter());
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

                // splits states into 2*n, one with the optional path in it and one without
                Edge::Optional(optional_path) => {
                    match Self::merge_states_with_path(&current_states, optional_path, tracker) {
                        MergeStatus::Added(mut ends) => {
                            ends.extend(current_states.into_iter());
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
                },

                Edge::Final(_) => {
                    unimplemented!()
                }
            };

            tracker.reserve_last_target_value();

            match status {
                MergeStatus::Added(states) => current_states = states,
                MergeStatus::Failed => panic!("aaah") // FIXME
            }
        }

        // Connecting last states with end state
        let rest = current_states.into_iter()
            .map(|mut state| state.borrow_mut().add_node(Self {
                value: 0,
                nodes: vec![],
                transition: '\0'
            }))
            .collect();
        MergeStatus::Added(rest)
    }

    pub fn all_combinations(&self) -> Vec<(String, usize)> {
        let transition = if self.transition.eq(&'\0') {
            String::new()
        } else {
            self.transition.to_string()
        };
        if self.nodes.len() == 0 {
            return vec![(transition, self.value)];
        }
        let mut combinations = vec![];

        for node in &self.nodes {
            combinations.extend(node.borrow().all_combinations().iter().map(|combination|
                ( format!("{}{}", transition, combination.0), node.borrow().value )))
        }
        combinations
    }
}

