use crate::edge::{Edge, EdgeType};

#[derive(PartialEq)]
pub struct State<'a> {
    pub edge_index: u16,
    pub children: Vec<Edge<'a>>
}

impl<'a> State<'a> {
    pub fn new() -> Self {
        Self { children: vec![], edge_index: 0 }
    }

    pub fn add_edge(&mut self, edge: EdgeType<'a>, do_merge: bool) -> (bool, &mut State<'a>) {
        match (do_merge, self.children.iter_mut().find(|child| child.data.eq(&edge))) {
            (true, Some(ref mut edge)) => {
                (true, &mut edge.to_state)
            },
            _ => {
                let edge_index = self.edge_index;

                self.edge_index += 1;

                let mut state = State::<'a>::new();
                let edge = Edge { incr_value: edge_index, to_state: state, data: edge };
                self.children.push(edge);
                (false, &mut state)
            }
        }
    }
}