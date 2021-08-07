use crate::{State, EdgeType};

pub struct FSM<'a> {
    start: State<'a>
}

impl<'a> FSM<'a> {
    pub fn build(config: &[&[EdgeType<'a>]]) -> Self {
        let mut start = State::new();

        for edges in config.iter() {
            let mut current = &mut start;
            let mut still_merging = true;

            for edge in edges.iter() {
                let (merged, new_state) = current.add_edge(edge.clone(), still_merging);
                still_merging = merged;
                current = new_state;
            }
        }

        Self {
            start
        }
    }
}