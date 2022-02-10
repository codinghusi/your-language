use std::convert::TryInto;
use std::rc::Rc;

use crate::builder::id_gen::IdGen;
use crate::builder::state::{MergeStatus, State};
use crate::builder::tracker::Tracker;
use crate::machine::machine::Context;
use crate::machine::machine::Machine;
use crate::path::Path;
use crate::FSM;

pub struct FSM_Builder {
    paths: Vec<Path>,
}

impl FSM_Builder {
    pub fn new() -> Self {
        Self { paths: vec![] }
    }

    pub fn from(paths: Vec<Path>) -> Self {
        Self { paths }
    }

    pub fn add(&mut self, path: Path) -> &mut Self {
        self.paths.push(path);
        self
    }

    pub fn build_machine(&self) -> Result<Machine, String> {
        Machine::from_paths(&self.paths)
    }
}
