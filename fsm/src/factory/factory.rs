use crate::fsm::FSM;
use crate::EdgeType;

pub struct FSM_Factory {

}

impl FSM_Factory {
    pub fn build<'a>(paths: Vec<Vec<EdgeType<'a>>>) -> FSM {
        let mut root = Item::new(EdgeType::Start);


        for items in paths {
            for item in items {
                let mut current = &mut start;
            }
        }

        //
        // let mut start = State::new();
        //
        // for edges in config.iter() {
        //     let mut current = &mut start;
        //     let mut still_merging = true;
        //
        //     for edge in edges.iter() {
        //         let (merged, new_state) = current.add_edge(edge.clone(), still_merging);
        //         still_merging = merged;
        //         current = new_state;
        //     }
        // }
        //
        // Self {
        //     start
        // }
    }
}