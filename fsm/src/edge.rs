use crate::state::State;
use std::cell::RefCell;

#[derive(PartialEq, Debug)]
pub struct Edge {
    pub value: i16,
    pub data: EdgeType,
    pub to_state: RefCell<State>,
    pub capture: Capture,
}

#[derive(PartialEq, Debug)]
pub enum Capture {
    None,
    Start,
    Stop
}

#[derive(PartialEq, Clone, Debug)]
pub enum EdgeType {
    // Start,
    Jump,
    Char(char),
}

#[derive(Debug)]
pub enum ParseStatus {
    Success,
    Again, // for Start and Capture that don't eat
    Failed
}

impl From<EdgeType> for Vec<EdgeType> {
    fn from(edge_type: EdgeType) -> Self {
        vec![edge_type]
    }
}