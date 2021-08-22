use crate::state_edge::StateEdge;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
pub struct FSM {
    pub root: Rc<RefCell<StateEdge>>
}

impl FSM {

}