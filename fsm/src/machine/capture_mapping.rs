use std::collections::HashMap;
use std::ops::Range;

use crate::machine::machine::{CaptureId, StateId};

pub struct CaptureMapping {
    pub item: ItemValue,
    pub capture: CaptureId,
}

pub enum ItemValue {
    List(Vec<ItemValue>),
    Map(HashMap<String, ItemValue>),
    String { value: String },
}

pub enum MappingResult {
    Specific(ItemValue),
    Implicit(Range<StateId>),
}
