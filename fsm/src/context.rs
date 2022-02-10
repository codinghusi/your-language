use std::collections::HashMap;

use super::capture_helpers::*;
use super::definitions::*;

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

    pub fn clone_without_items(&self) -> Self {
        Context {
            items: HashMap::new(),
            is_in_cycle: self.is_in_cycle,
            target_state: self.target_state,
        }
    }
}
