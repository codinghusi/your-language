use super::*;

impl Machine {
    /// returns id of added capture
    pub fn add_capture(&mut self) -> usize {
        let capture = self.capture_count;
        self.capture_count += 1;
        capture
    }

    /// returns id of added state
    pub fn add_state(&mut self) -> usize {
        let state = self.state_count;
        self.transition_table.insert(state, [0; 255]);
        self.state_count += 1;
        state
    }

    fn get_transition(&self, state: &usize) -> Result<&TransitionFunction, String> {
        self.transition_table.get(state).ok_or(format!(
            "state '{}.' doesn't exist. There are only {} states available",
            state, self.state_count
        ))
    }

    // TODO: return Err if transition has error_state
    pub(super) fn get_transition_at(
        &self,
        state: &usize,
        transition: char,
    ) -> Result<&usize, String> {
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

    pub(super) fn apply_transitions(
        &mut self,
        source: &usize,
        destination: &usize,
    ) -> Result<(), String> {
        let source_table = self.get_transition(source)?.clone();

        source_table
            .iter()
            .enumerate()
            .filter(|(_, target)| **target != ERROR_STATE)
            .map(|(c, target)| self.set_transition(destination, c as u8 as char, *target))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(())
    }
}
