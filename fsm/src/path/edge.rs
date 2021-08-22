
use super::{Capture, Path};

pub enum Edge {
    Char(char),
    OneOf(Vec<Path>),
    Optional(Path),
    Capture(Capture),
    Cycle(Path)
}