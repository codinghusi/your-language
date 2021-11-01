use std::collections::{HashSet, HashMap};
use crate::path::Path;

// Note: a state is only a unique id (number counting from 0 to usize::max_value)
// TODO: state 0 needs to be an error catching state

const ERROR_STATE: usize = 0;
type TransitionFunction = [usize; 255];

#[derive(Debug)]
pub struct Machine {
    size: usize,                    // Q: set of all states (actually only the count of them: 0..states)
    start_state: usize,             // q0: initial state (reference to it)
    final_states: HashSet<usize>,   // F: set of final states (references to it)
    transition_table: HashMap<usize, TransitionFunction> // delta: all transition functions
}

// Note: First state is the error state
impl Machine {
    pub fn empty() -> Self {
        let mut new = Self {
            size: 1,
            start_state: 0,
            final_states: HashSet::new(),
            transition_table: HashMap::new()
        };
        new.add_state(); // add the root state. Currently always with id: 1
        new
    }

    pub fn from_path(path: &Path) -> Self {
        let mut machine = Self::empty();
        let state = machine.add_state();
        machine.insert_path_at(&state, path);
        machine
    }

    // FIXME: this is deprecated, we have multiple start_states. Consider forcing only one starting state
    pub fn get_root_state(&self) -> &usize {
        &1
    }

    /// returns id of added state
    pub fn add_state(&mut self) -> usize {
        let state = self.size;
        self.transition_table.insert(state, [0; 255]);
        self.size += 1;
        state
    }

    fn get_transition(&self, state: &usize) -> Result<&TransitionFunction, String> {
        self.transition_table.get(state).ok_or(format!("state '{}.' doesn't exist. There are only {} states available", state, self.size))
    }

    fn get_transition_at(&self, state: &usize, transition: char) -> Result<&usize, String> {
        let transition_function = self.get_transition(state)?;
        Ok(&transition_function[transition as usize])
    }

    fn get_mut_transition_at(&mut self, state: &usize, transition: char) -> Result<&mut usize, String> {
        let transition_function = self.get_transition_function_mut(state)?;
        Ok(&mut transition_function[transition as usize])
    }

    fn get_transition_function_mut(&mut self, state: &usize) -> Result<&mut TransitionFunction, String> {
        self.transition_table.get_mut(&state).ok_or(format!("state '{}.' doesn't exist. There are only {} states available", state, self.size))
    }

    /// let 'state' transition to 'destination' when char is the given input
    pub fn set_transition(&mut self, state: &usize, transition: char, destination: usize) -> Result<(), String> {
        let current_destination = self.get_transition_at(state, transition)?;
        if *current_destination != ERROR_STATE {
            return Err(format!("state '{}.' already transitions to state '{}.' by char {}. It can't also transition to '{}.'", state, current_destination, transition, destination))
        }
        *(self.get_mut_transition_at(state, transition)?) = destination;
        Ok(())
    }

    pub fn setup_transition(&mut self, state: &usize, transition: char) -> Result<usize, String> {
        let current_destination = self.get_transition_at(state, transition)?;
        if *current_destination != ERROR_STATE {
            Ok(*current_destination)
        } else {
            let destination = self.add_state();
            *(self.get_mut_transition_at(state, transition)?) = destination;
            Ok(destination)
        }
    }

    pub fn insert_path_at(&mut self, state: &usize, path: &Path) -> Result<Vec<usize>, String> {
        self.insert_path_at_states(vec![*state], path)
    }

    // TODO: change states: Vec<_> into HashSet
    /// merges the Path into all the given states (sometimes recursively)
    pub fn insert_path_at_states(&mut self, states: Vec<usize>, path: &Path) -> Result<Vec<usize>, String> {
        let mut current_states = states;
        for item in &path.items {
            // merge current path item into all given states
            // also get the new list of all next states that need merging. (state_indexes.len() >= previous length)
            use crate::path::Edge::*;
            current_states = match item {
                Char(c) => {
                    current_states.iter().map(|state| self.setup_transition(state, *c)).collect::<Result<_, _>>()?
                },
                OneOf(paths) => {
                    paths.iter()
                        .map(|path| current_states.iter()
                            .map(|state| self.insert_path_at(state, path))
                            .collect::<Result<_, _>>()
                        )
                        .collect::<Result<Vec<Vec<_>>, _>>()?
                        .into_iter().flatten().flatten().collect()
                },
                _ => unimplemented!("given path item not implemented")
            }
        }
        Ok(current_states)
    }

    pub fn is_ready(&self) -> bool {
        self.size > 1
    }

    pub fn all_combinations(&self) -> Vec<String> {
        self.all_combinations_internal(self.get_root_state())
    }

    fn all_combinations_internal(&self, state: &usize) -> Vec<String> {
        let result: Vec<_> = self.get_transition(state).unwrap().iter().enumerate()
            .filter(|(c, destination)| **destination != ERROR_STATE)
            .flat_map(|(c, destination)| self.all_combinations_internal(destination).iter()
                .map(
                    |str| format!("{}{}", c as u8 as char, str)
                )
                .collect::<Vec<_>>()
            )
            .collect();
        if result.len() == 0 {
            vec!["".to_string()]
        } else {
            result
        }
    }
}