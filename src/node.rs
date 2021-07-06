
pub trait NodeType: Sized {
    fn get_type(&self) -> String;
}
