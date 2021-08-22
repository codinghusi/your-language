use crate::path::Path;

pub enum Capture {
    Struct {
        key: String,
        path: Path
    },
    Text {
        key: String,
        path: Path
    }
}