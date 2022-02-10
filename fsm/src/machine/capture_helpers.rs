use super::types::*;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::hash::{Hash, Hasher};

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

#[derive(Debug, Clone)]
pub enum CaptureValue {
    String(CaptureId),
    List(Box<CaptureValue>),
    Map(HashMap<String, CaptureValue>),
}

pub enum CapturedType {
    Value(CapturedValue),
    List(Vec<CapturedValue>),
}

impl fmt::Debug for CapturedType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CapturedType::Value(value) => write!(f, "{:?}", value),
            CapturedType::List(list) => write!(
                f,
                "[{}]",
                list.iter()
                    .map(|v| format!("{:?}", v))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        }
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct PendingCapture {
    pub payload: CapturePayload,
    pub start_index: usize,
}

#[derive(Debug)]
pub struct CapturedValue {
    pub capture_id: CaptureId,
    pub value: String,
}
