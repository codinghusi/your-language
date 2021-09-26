use crate::builder::id_gen::IdGen;
use crate::FSM;
use crate::path::Path;
use crate::builder::state::{State, MergeStatus};
use std::rc::Rc;
use std::convert::TryInto;
use crate::builder::tracker::Tracker;

pub struct FSM_Builder {
    paths: Vec<Path>
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
                    _ => ()
                }
            });
        FSM { root }
    }
}