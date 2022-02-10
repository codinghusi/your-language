use crate::path::capture::CaptureItem;

use super::Path;

#[derive(Clone)]
pub enum Edge {
    Char(char),
    OneOf(Vec<Path>),
    Optional(Path),
    Capture(CaptureItem),
    Cycle(Path),
    Final,
}
