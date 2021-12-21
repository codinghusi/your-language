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
        let mut machine = Machine::empty();
        let root = *machine.get_root_state();
        for path in &self.paths {
            machine.insert_path_at(&root, path, &mut Context::new())?;
        }
        Ok(machine)
    }

    pub fn build(&self) -> FSM {
        let mut root = State::new_root();
        let mut ids = IdGen::new();
        let mut tracker = Tracker::new();
        self.paths
            .iter()
            .map(|path| State::merge_path(root.clone(), path, &mut tracker.clone()))
            .for_each(|status| {
                match status {
                    MergeStatus::Failed => panic!("building failed..."), // TODO: make this more proper into a Result<>
                    _ => (),
                }
            });
        FSM { root }
    }
}
