use super::*;

impl Machine {
    pub fn is_ready(&self) -> bool {
        self.state_count > 1
    }

    pub fn get_root_state(&self) -> &usize {
        &1
    }
}
