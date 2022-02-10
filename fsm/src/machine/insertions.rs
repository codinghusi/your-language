use super::*;

// Note: State 0 is the error state
impl Machine {
    pub fn from_path(path: Path) -> Result<Self, String> {
        Self::from_paths(&vec![path])
    }

    pub fn from_paths(paths: &Vec<Path>) -> Result<Self, String> {
        let mut machine = Self::empty();
        let state = *machine.get_root_state();
        let mut context = Context::new();
        machine.insert_paths_at_states(vec![state], paths, &mut context)?;
        machine.capture_structure_root = context.items;
        Ok(machine)
    }

    pub fn insert_path_at(
        &mut self,
        state: &usize,
        path: &Path,
        context: &mut Context,
    ) -> Result<Vec<usize>, String> {
        self.insert_path_at_states(vec![*state], path, context)
    }

    pub fn insert_paths_at_states(
        &mut self,
        states: Vec<usize>,
        paths: &Vec<Path>,
        context: &mut Context,
    ) -> Result<Vec<usize>, String> {
        Ok({
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
                let previous_target_state = context.target_state;
                context.target_state = Some(*closing);

                for path in paths {
                    other_lose_ends.append(&mut self.insert_path_at_states(
                        states.clone(),
                        path,
                        context,
                    )?);
                }
                first_lose_ends.append(&mut other_lose_ends);

                context.target_state = previous_target_state;
                first_lose_ends
            } else {
                vec![]
            }
        })
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
            OneOf(paths) => self.insert_paths_at_states(states, paths, context)?,
            Optional(path) => {
                // Paths with 'path' and paths without 'path' (skipped)
                let mut lose_ends = self.insert_path_at_states(states.clone(), path, context)?;
                lose_ends.append(&mut states.clone());
                lose_ends
            }
            Final => {
                states.clone().into_iter().for_each(|state| {
                    self.final_states.insert(state);
                });
                states
            }
            Cycle(path) => {
                context.is_in_cycle = true;

                let lose_ends = self.insert_path_at_states(states.clone(), path, context)?;
                for begin in &states {
                    for end in &lose_ends {
                        self.apply_transitions(begin, end)?;
                    }
                }

                context.is_in_cycle = false;

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
                            CapturePayload {
                                capture_id: capture,
                                end_states: ends.clone().into_iter().collect(),
                                is_list: context.is_in_cycle,
                            },
                        );
                        lose_ends.append(&mut ends);
                    }

                    // setup mapping
                    let value = CaptureValue::String(capture);
                    if context.is_in_cycle {
                        context
                            .items
                            .insert(String::from(&item.key), CaptureValue::List(Box::new(value)));
                    } else {
                        context.items.insert(String::from(&item.key), value);
                    }

                    lose_ends
                }
                CaptureType::Struct => {
                    let mut new_context = context.clone_without_items();
                    let is_in_cycle = context.is_in_cycle;
                    new_context.is_in_cycle = false;
                    let ret = self.insert_path_at_states(states, &item.path, &mut new_context)?;
                    let value = CaptureValue::Map(new_context.items);
                    if is_in_cycle {
                        context
                            .items
                            .insert(String::from(&item.key), CaptureValue::List(Box::new(value)));
                    } else {
                        context.items.insert(String::from(&item.key), value);
                    }

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
        let prev_target_state = context.target_state;
        context.target_state = None;
        for edge in path.items.iter().take(path.items.len() - 1) {
            current_states = self.insert_edge_at_states(current_states, &edge, context)?;
        }
        let last = path.items.last().unwrap(); // TODO: this .unwrap() could be done unchecked
        context.target_state = prev_target_state;
        current_states = self.insert_edge_at_states(current_states, last, context)?;
        Ok(current_states)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn table(transitions: &[(char, usize)]) -> TransitionFunction {
        let mut transition_table = [0usize; 255];
        for t in transitions {
            transition_table[t.0 as usize] = t.1;
        }
        transition_table
    }

    // that shit is complicated, lol
    // #[test]
    fn string() -> Result<(), String> {
        let machine = Machine::from_path(Path::new().string("abc").end())?;
        let mut final_states = HashSet::new();
        final_states.insert(4);
        let mut transition_table = HashMap::new();
        transition_table.insert(1, table(&[('a', 2)]));
        transition_table.insert(2, table(&[('b', 3)]));
        transition_table.insert(3, table(&[('c', 4)]));
        assert_eq!(
            machine,
            Machine {
                state_count: 4,
                start_state: 1,
                final_states,
                transition_table,
                capture_count: 0,
                capture_table: Default::default(),
                capture_structure_root: Default::default(),
            },
        );
        Ok(())
    }
}
