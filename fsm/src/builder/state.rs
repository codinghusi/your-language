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

    pub fn get_node_by_transition(&self, transition: &char) -> Option<&StateRef> {
        self.nodes.iter().find(|node| node.borrow().transition.eq(transition))
    }

    pub fn add_node_by_transition(&mut self, transition: &char, target_value: i16) -> (StateRef, i16) {
        println!("adding {}-{}", self.transition, transition);
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

    pub fn merge_path(root: StateRef, path: &Path, counter: &mut IdGen) -> MergeStatus {
        Self::merge_items(root, path.items.iter(), counter)
    }

    fn merge_states_with_path(states: &HashSet<StateRef>, path: &Path, counter: &mut IdGen) -> MergeStatus {
        let mut failed = false;
        // let result: Result<HashSet<StateRef>, MergeStatus> = states
        let result: Result<Vec<_>, _> = states
            .iter()
            .map(|state| Self::merge_path(state.clone(), path, counter))
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

    pub fn merge_items(root: StateRef, items: Iter<Edge>, counter: &mut IdGen) -> MergeStatus {
        let mut capture_tracker = CaptureTracker::new();
        let mut target_value = counter.next();
        let mut current_states = HashSet::new();
        current_states.insert(root);
        for item in items {

            let status = match item {
                Edge::Char(transition) => {
                    // FIXME: adjust the target_value!
                    println!("adding ends: {}", current_states.iter().map(|state| state.borrow().transition.to_string()).collect::<Vec<_>>().join(", "));

                    let states = current_states.into_iter()
                        .map(|mut state| state.borrow_mut().add_node_by_transition(transition, target_value as i16))
                        .map(|(state, _)| state) // follow up of fix me: the _ represents the remainder target_value
                        .collect();
                    MergeStatus::Added(states)
                },

                Edge::OneOf(possible_paths) => {
                    let mut all_ends = HashSet::new();
                    let succeed = possible_paths
                        .iter()
                        .map(|path| Self::merge_states_with_path(&current_states, path, counter))
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

                Edge::Optional(optional_path) => {
                    println!("opt adding {:?}", current_states.iter().map(|state| state.borrow().transition.to_string()).collect::<Vec<_>>());
                    match Self::merge_states_with_path(&current_states, optional_path, counter) {
                        MergeStatus::Added(mut ends) => {
                            println!("opt merging {:?}", ends.iter().map(|state| state.borrow().transition.to_string()).collect::<Vec<_>>());
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
                }
            };

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

