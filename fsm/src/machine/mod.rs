use std::collections::{HashMap, HashSet};
use std::iter::Peekable;

use crate::path::{CaptureType, Edge, Path};

use super::capture_helpers::*;
use super::context::Context;
use super::definitions::*;

mod export_xstate_js;
mod insertions;
mod other;
mod parsing;
mod tests;
mod to_json;
mod transitions_and_captures;

// Note: a state is only a unique id (number counting from 0 to usize::max_value)
// TODO: state 0 needs to be an error catching state

#[derive(Debug)]
pub struct Machine {
    state_count: usize,
    // Q: set of all states (actually only the count of them: 0..states)
    start_state: usize,
    // q0: initial state (reference to it)
    final_states: HashSet<usize>,
    // F: set of final states (references to it)
    transition_table: HashMap<usize, TransitionFunction>, // delta: all transition functions

    // TODO: this part can be another struct
    // captures will record characters to output them later as tree
    // capture_mapping: CaptureMapping,  // mapping will be implemented later // for mapping the key-value/flat structures of captures into a tree-structure
    capture_count: usize,
    // tracks how many caputures are used. Used to generate auto-increment ids for captures
    capture_table: HashMap<StateId, CapturePayload>,
    // <start_id, (end_ids, capture_id)> // connects different states with captures, provides a fast way to know which characters need to be capatured
    capture_structure_root: HashMap<String, CaptureValue>,
}

impl Machine {
    pub fn empty() -> Self {
        let mut new = Self {
            state_count: 1,
            start_state: 1,
            final_states: HashSet::new(),
            transition_table: HashMap::new(),

            capture_table: HashMap::new(),
            capture_count: 0,
            capture_structure_root: HashMap::new(),
        };
        new.add_state(); // add the root state. Currently always with id: 1
        new
    }
}
