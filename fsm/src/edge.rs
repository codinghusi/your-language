use crate::state::State;

#[derive(PartialEq)]
pub struct Edge<'a> {
    pub incr_value: u16,
    pub data: EdgeType<'a>,
    pub to_state: State<'a>
}

#[derive(Clone)]
pub enum EdgeType<'a> {
    Start,
    Capture {

    },
    Char(char),
    Loop(&'a State<'a>)
}

impl PartialEq for EdgeType<'_> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Char(c1), Self::Char(c2)) if c1.eq(c2) => true,
            (Self::Capture {..}, Self::Capture {..}) => false,
            _ => false
        }
    }
}