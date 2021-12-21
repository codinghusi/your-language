use std::cell::RefCell;
use std::rc::Rc;

use crate::builder::state::{State, StateRef};

#[derive(Debug)]
pub struct FSM {
    pub root: StateRef,
}

impl FSM {}
