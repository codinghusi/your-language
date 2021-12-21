use std::collections::HashMap;

use super::capture_mapping::CaptureMapping;
use super::machine::StateId;

pub enum CaptureValue {
    Number { state: StateId },
    String { state: StateId },
    List { values: Vec<CaptureValue> },
    Map(HashMap<String, CaptureMapping>),
}

pub struct CaptureContext {
    current: Option<CaptureValue>,
    key: Option<String>,
    state: Option<StateId>,
    nested_cycles: usize,
}

impl CaptureContext {
    pub fn new() -> Self {
        Self {
            current: None,
            key: None,
            state: None,
            nested_cycles: 0,
        }
    }

    pub fn set(&mut self, key: String, value: CaptureValue) -> &mut Self {
        if let None = self.current {
            if self.nested_cycles > 0 {
                while self.nested_cycles > 0 {
                    self.nested_cycles -= 1;
                }
            }
        }

        if let Some(current) = &self.current {
        } else {
            unreachable!();
        }

        self
    }

    pub fn entering_a_cycle(&mut self) {
        self.nested_cycles += 1;
    }

    pub fn leaving_a_cycle(&mut self) {
        if self.nested_cycles == 0 {
            panic!("caller says: 'leaving a cycle', while it we aren't in a cycle");
        }
        self.nested_cycles -= 1;
    }
}
