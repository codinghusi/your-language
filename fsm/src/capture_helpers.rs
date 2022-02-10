use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

use super::definitions::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapturePayload {
    pub capture_id: CaptureId,
    pub end_states: HashSet<StateId>,
    pub is_list: bool,
}

impl Hash for CapturePayload {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.capture_id.hash(state);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CaptureValue {
    String(CaptureId),
    List(Box<CaptureValue>),
    Map(HashMap<String, CaptureValue>),
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct PendingCapture {
    pub payload: CapturePayload,
    pub start_index: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub struct CapturedValue {
    pub capture_id: CaptureId,
    pub value: String,
}
