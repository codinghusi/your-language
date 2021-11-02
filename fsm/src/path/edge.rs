
use super::{Path};
use crate::path::capture::CaptureItem;

pub enum Edge {
    Char(char),
    OneOf(Vec<Path>),
    Optional(Path),
    Capture(CaptureItem),
    Cycle(Path),
    Final(usize)
}