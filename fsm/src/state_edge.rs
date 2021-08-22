use crate::path::{Path, Edge};
use std::ops::RangeFrom;
use std::rc::Rc;
use std::cell::RefCell;
use std::slice::Iter;

pub enum MergeStatus {
    Added(Vec<Rc<RefCell<StateEdge>>>),
    Failed
}

#[derive(Debug)]
pub struct StateEdge {
    pub value: i16,
    pub nodes: Vec<Rc<RefCell<StateEdge>>>,
    pub transition: char
}

impl StateEdge {
    pub fn new_root() -> Rc<RefCell<Self>> {
        // FIXME: it's dirty, because value and transition aren't used here
        Rc::new(RefCell::new(Self {
            value: 0,
            nodes: vec![],
            transition: '\0'
        }))
    }

    pub fn id_gen() -> RangeFrom<i16> {
        (0..).into_iter()
    }

    pub fn add_node(&mut self, node: StateEdge) -> Rc<RefCell<Self>> {
        let rc = Rc::new(RefCell::new(node));
        self.nodes.push(Rc::clone(&rc));
        rc
    }

    pub fn get_node_by_transition(&mut self, transition: &char) -> Option<&Rc<RefCell<StateEdge>>> {
        self.nodes.iter().find(|node| node.borrow().transition.eq(transition))
    }

    pub fn add_node_by_transition(&mut self, transition: &char, target_value: i16) -> (Rc<RefCell<Self>>, i16) {
        match self.get_node_by_transition(transition) {
            Some(cell) => (cell.clone(), target_value),
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
    pub fn merge_path(root: Rc<RefCell<Self>>, path: &Path, counter: &mut RangeFrom<i16>) -> MergeStatus{
        Self::merge_items(root, path.items.iter(), counter)
    }

    fn merge_states_with_path(states: &Vec<Rc<RefCell<StateEdge>>>, path: &Path, counter: &mut RangeFrom<i16>) -> MergeStatus {
        let mut end_states = vec![];
        let result = states.iter()
            .map(|state| Self::merge_path(Rc::clone(state), path, counter))
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

    pub fn merge_items(root: Rc<RefCell<Self>>, items: Iter<Edge>, counter: &mut RangeFrom<i16>) -> MergeStatus {
        let mut target_value = counter.next().unwrap();
        let mut current_states = vec![root];
        for item in items {
            let status = match item {
                Edge::Char(transition) => {
                    // FIXME: not adjusting target_value!
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
                        .any(|mut status| match status {
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
                }
                Edge::Optional(optional_path) => {
                    match Self::merge_states_with_path(&current_states, optional_path, counter) {
                        MergeStatus::Added(mut ends) => {
                            ends.append(&mut current_states);
                            MergeStatus::Added(ends)
                        },
                        fail => fail
                    }
                }
                Edge::Capture(_) => {
                    unimplemented!()
                }
                Edge::Cycle(_) => {
                    unimplemented!()
                }
            };

            match status {
                MergeStatus::Added(states) => current_states = states,
                MergeStatus::Failed => panic!("aaah")
            }
        }
        MergeStatus::Added(current_states)
    }
}

