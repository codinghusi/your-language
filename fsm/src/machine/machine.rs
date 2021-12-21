// use super::capture_mapping::{ItemValue, MappingResult};
use crate::path::{CaptureType, Edge, Path};
use std::collections::{HashMap, HashSet};

// Note: a state is only a unique id (number counting from 0 to usize::max_value)
// TODO: state 0 needs to be an error catching state

const ERROR_STATE: usize = 0;

pub type TransitionFunction = [usize; 255];
pub type StateId = usize;
pub type CaptureId = usize;

#[derive(Debug)]
pub struct CaptureEnds {
    pub capture_id: CaptureId,
    pub end_states: Vec<StateId>,
}

#[derive(Debug, Clone)]
pub enum CaptureValue {
    String(CaptureId),
    List(CaptureId),
    Map(HashMap<String, CaptureValue>),
}

#[derive(Clone)]
pub struct Context {
    pub items: HashMap<String, CaptureValue>,
    pub is_in_cycle: bool,
    pub target_state: Option<StateId>,
}

impl Context {
    pub fn new() -> Self {
        Context {
            items: HashMap::new(),
            is_in_cycle: false,
            target_state: None,
        }
    }

    pub fn with_target_state(&self, state: StateId) -> Self {
        Context {
            items: self.items.clone(),
            is_in_cycle: self.is_in_cycle,
            target_state: Some(state),
        }
    }

    pub fn without_target_state(&self) -> Self {
        Context {
            items: self.items.clone(),
            is_in_cycle: self.is_in_cycle,
            target_state: None,
        }
    }

    pub fn without_items(&self) -> Self {
        Context {
            items: HashMap::new(),
            is_in_cycle: self.is_in_cycle,
            target_state: self.target_state,
        }
    }
}

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
    capture_table: HashMap<StateId, CaptureEnds>, // <start_id, (end_ids, capture_id)> // connects different states with captures, provides a fast way to know which characters need to be capatured
}

// Note: First state is the error state
impl Machine {
    pub fn empty() -> Self {
        let mut new = Self {
            state_count: 1,
            start_state: 1,
            final_states: HashSet::new(),
            transition_table: HashMap::new(),

            capture_table: HashMap::new(),
            capture_count: 0,
        };
        new.add_state(); // add the root state. Currently always with id: 1
        new
    }

    pub fn from_path(path: &Path) -> Self {
        let mut machine = Self::empty();
        let state = machine.add_state();
        machine.insert_path_at(&state, path, &mut Context::new());
        machine
    }

    // FIXME: this is deprecated, we have multiple start_states. Consider forcing only one starting state
    pub fn get_root_state(&self) -> &usize {
        &1
    }

    /// returns id of added state
    pub fn add_state(&mut self) -> usize {
        let state = self.state_count;
        self.transition_table.insert(state, [0; 255]);
        self.state_count += 1;
        state
    }

    /// returns id of added capture
    pub fn add_capture(&mut self) -> usize {
        let capture = self.capture_count;
        self.capture_count += 1;
        capture
    }

    fn get_transition(&self, state: &usize) -> Result<&TransitionFunction, String> {
        self.transition_table.get(state).ok_or(format!(
            "state '{}.' doesn't exist. There are only {} states available",
            state, self.state_count
        ))
    }

    // TODO: return Err if transition has error_state
    fn get_transition_at(&self, state: &usize, transition: char) -> Result<&usize, String> {
        let transition_function = self.get_transition(state)?;
        Ok(&transition_function[transition as usize])
    }

    fn get_mut_transition_at(
        &mut self,
        state: &usize,
        transition: char,
    ) -> Result<&mut usize, String> {
        let transition_function = self.get_transition_function_mut(state)?;
        Ok(&mut transition_function[transition as usize])
    }

    fn get_transition_function_mut(
        &mut self,
        state: &usize,
    ) -> Result<&mut TransitionFunction, String> {
        self.transition_table.get_mut(&state).ok_or(format!(
            "state '{}.' doesn't exist. There are only {} states available",
            state, self.state_count
        ))
    }

    /// let 'state' transition to 'destination' when char is the given input
    pub fn set_transition(
        &mut self,
        state: &usize,
        transition: char,
        destination: usize,
    ) -> Result<(), String> {
        let current_destination = self.get_transition_at(state, transition)?;
        if *current_destination != ERROR_STATE {
            if *current_destination == destination {
                return Ok(());
            }
            return Err(format!("state '{}.' already transitions to state '{}.' by char {}. It can't also transition to '{}.'", state, current_destination, transition, destination));
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

    pub fn insert_path_at(
        &mut self,
        state: &usize,
        path: &Path,
        context: &mut Context,
    ) -> Result<Vec<usize>, String> {
        self.insert_path_at_states(vec![*state], path, context)
    }

    pub fn insert_edge_at_states(
        &mut self,
        states: Vec<usize>,
        item: &Edge,
        context: &mut Context,
    ) -> Result<Vec<usize>, String> {
        // merge edge into all given states
        // also get the new list of all next states that need merging.
        use crate::path::Edge::*;
        let end_states = Ok(match item {
            Char(c) => {
                let mut to_state = context.target_state;
                states
                    .iter()
                    .map(|state| {
                        if let Some(new_state) = to_state {
                            self.set_transition(state, *c, new_state)?;
                        } else {
                            to_state = Some(self.setup_transition(state, *c)?);
                        }
                        Ok(())
                    })
                    .collect::<Result<(), String>>()?;
                if let Some(new_state) = to_state {
                    vec![new_state]
                } else {
                    vec![]
                }
            }
            OneOf(paths) => {
                let mut paths = paths.iter();
                if let Some(first) = paths.next() {
                    // Step 1: insert the first path
                    let mut other_lose_ends = vec![];
                    let mut first_lose_ends =
                        self.insert_path_at_states(states.clone(), first, context)?;

                    // Step 2: Grab any end state, into those all other paths will be merged into
                    let closing = first_lose_ends
                        .first()
                        .ok_or("it's illegal to provide an empty path!".to_string())?; // TODO: add this note to the documentation later on

                    // Step 3: insert all path with end state as the same as the first path
                    let mut new_context = context.with_target_state(*closing);

                    for path in paths {
                        other_lose_ends.append(&mut self.insert_path_at_states(
                            states.clone(),
                            path,
                            &mut new_context,
                        )?);
                    }
                    first_lose_ends.append(&mut other_lose_ends);
                    first_lose_ends
                } else {
                    vec![]
                }
            }
            Optional(path) => {
                // Paths with 'path' and paths without 'path' (skipped)
                let mut lose_ends = self.insert_path_at_states(states.clone(), path, context)?;
                lose_ends.append(&mut states.clone());
                lose_ends
            }
            Final(value) => {
                states.clone().into_iter().for_each(|state| {
                    self.final_states.insert(state);
                });
                states
            }
            Cycle(path) => {
                let lose_ends = self.insert_path_at_states(states.clone(), path, context)?;
                states.iter().for_each(|begin| {
                    lose_ends.iter().for_each(|end| {
                        self.apply_transitions(begin, end);
                    })
                });

                lose_ends
            }
            Capture(item) => match item.ty {
                CaptureType::Text => {
                    let mut lose_ends = vec![];
                    let capture = self.add_capture();

                    // setup capturing
                    for start in states {
                        let mut ends = self.insert_path_at(&start, &item.path, context)?;
                        self.capture_table.insert(
                            start,
                            CaptureEnds {
                                capture_id: capture,
                                end_states: ends.clone(),
                            },
                        );
                        lose_ends.append(&mut ends);
                    }

                    // setup mapping
                    context
                        .items
                        .insert(String::from(&item.key), CaptureValue::String(capture));

                    lose_ends
                }
                CaptureType::Struct => {
                    let mut new_context = context.without_items();
                    let ret = self.insert_path_at_states(states, &item.path, &mut new_context)?;
                    context.items.insert(
                        String::from(&item.key),
                        CaptureValue::Map(new_context.items),
                    );
                    ret
                }
            },
        });

        end_states
    }

    // TODO: change states: Vec<_> into HashSet
    /// merges the Path into all the given states (sometimes recursively)
    pub fn insert_path_at_states(
        &mut self,
        states: Vec<usize>,
        path: &Path,
        context: &mut Context,
    ) -> Result<Vec<usize>, String> {
        if path.items.len() == 0 {
            return Ok(states); // TODO: could also throw an error that path.items are empty
        }
        let mut current_states = states;
        for edge in path.items.iter().take(path.items.len() - 1) {
            current_states = self.insert_edge_at_states(
                current_states,
                &edge,
                &mut context.without_target_state(),
            )?;
        }
        let last = path.items.last().unwrap(); // TODO: this .unwrap() could be done unchecked
        current_states = self.insert_edge_at_states(current_states, last, context)?;
        Ok(current_states)
    }

    pub fn interpret_slow(&self, text: &str) -> Result<Vec<(CaptureId, String)>, String> {
        let mut captures = vec![];
        let mut pending_captures: HashSet<(usize, StateId, CaptureId)> = HashSet::new();
        let mut current_state = self.start_state;

        for (i, c) in text.chars().enumerate() {
            // stop capture when needed
            let to_be_stopped = pending_captures
                .iter()
                .filter(|(_, end, _)| current_state > *end);
            for record in to_be_stopped.clone() {
                let (start_index, end_state, capture_id) = record;
                captures.push((*capture_id, String::from(&text[*start_index..i - 1])));
            }
            // remove 'em from the list
            pending_captures = pending_captures
                .into_iter()
                .filter(|(_, end, _)| current_state <= *end)
                .collect();

            // start capture when needed
            if let Some(marker) = self.capture_table.get(&current_state) {
                let CaptureEnds {
                    capture_id,
                    end_states,
                } = marker;
                for state in end_states {
                    pending_captures.insert((i, *state, *capture_id));
                }
                println!("start: {:?}", pending_captures);
            }

            // get next state
            // println!("transition: {:?}", self.get_transition(&current_state).unwrap());
            if let Ok(state) = self.get_transition_at(&current_state, c) {
                if *state != ERROR_STATE {
                    current_state = *state;
                    continue;
                }
            }

            return Err(format!("invalid character '{}'", c));
        }

        // collect the missed captures
        for record in pending_captures
            .iter()
            .filter(|(_, end, _)| current_state >= *end)
        {
            let (start_index, end_state, capture_id) = record;
            captures.push((
                *capture_id,
                String::from(&text[*start_index..text.len() - 1]),
            ));
        }

        Ok(captures)
    }

    fn apply_transitions(&mut self, source: &usize, destination: &usize) -> Result<(), String> {
        let source_table = self.get_transition(source)?.clone();

        source_table
            .iter()
            .enumerate()
            .filter(|(c, target)| **target != ERROR_STATE)
            .map(|(c, target)| self.set_transition(destination, c as u8 as char, *target))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(())
    }

    pub fn is_ready(&self) -> bool {
        self.state_count > 1
    }

    pub fn all_combinations(&self) -> Vec<String> {
        self.all_combinations_internal(self.get_root_state(), &mut HashSet::new())
    }

    // not working well with cycles
    fn all_combinations_internal(
        &self,
        state: &usize,
        visited: &mut HashSet<usize>,
    ) -> Vec<String> {
        if visited.contains(state) {
            return vec![];
        }
        visited.insert(*state);
        let mut result: Vec<_> = self
            .get_transition(state)
            .unwrap()
            .iter()
            .enumerate()
            .filter(|(c, destination)| **destination != ERROR_STATE)
            .flat_map(|(c, destination)| {
                self.all_combinations_internal(destination, visited)
                    .iter()
                    .map(|str| format!("{}{}", c as u8 as char, str))
                    .collect::<Vec<_>>()
            })
            .collect();
        if self.final_states.contains(state) {
            result.push("".to_string());
        }
        result
    }

    pub fn export_xstatejs(&self) -> String {
        let mut states = self.transition_table.iter().collect::<Vec<_>>();
        states.sort_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap());
        let states = states
            .iter()
            .map(|(state, func)| {
                let transitions = func
                    .iter()
                    .enumerate()
                    .filter(|(c, target)| **target != ERROR_STATE)
                    .map(|(c, target)| format!("'{}': '{}'", c as u8 as char, target))
                    .collect::<Vec<_>>()
                    .join(", ");
                let stateType = if self.final_states.contains(state) {
                    "type: 'final'"
                } else {
                    ""
                };
                format!(
                    "\t'{}': {{ on: {{ {} }}, {} }}",
                    state, transitions, stateType
                )
            })
            .collect::<Vec<_>>()
            .join(",\n");

        format!(
            r"import {{ createMachine }} from 'xstate';

const machine = createMachine({{
  id: 'machine',
  initial: '1',
  states: {{
{}
  }}
}});",
            states
        )
    }
}
