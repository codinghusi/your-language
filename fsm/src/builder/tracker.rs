use std::range::RangeFrom;
use super::id_gen::IdGen;

pub struct TrackingCore {
    id_gen: IdGen,
}

impl TrackingCore {
    pub fn new() -> Self {
        Self {
            id_gen: IdGen::new()
        }
    }

    pub fn next_target_value(&mut self) -> usize {
        counter.next().unwrap()
    }

    pub fn add_capture() {
        unimplemented!();
    }
}

type TrackingCoreRef = Rc<RefCell<TrackingCore>>;

#[derive(Clone)]
pub struct Tracker {
    core: TrackingCoreRef,
    index: IdGen
}

impl Tracker {
    pub fn new(core: TrackingCore) -> Self {
        Self {
            core,
            index: IdGen::new()
        }
    }
}