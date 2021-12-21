use std::borrow::Borrow;
use std::cell::RefCell;
use std::ops::Range;
use std::rc::Rc;

use super::id_gen::IdGen;
use super::tracking_core::{TrackedCapture, TrackingCore};

type TrackingCoreRef = Rc<RefCell<TrackingCore>>;

#[derive(Clone)]
pub struct Tracker {
    core: TrackingCoreRef,
    index: usize,
}

impl Tracker {
    pub fn new() -> Self {
        Self::new_with_core(TrackingCore::new())
    }

    pub fn new_with_core(core: TrackingCore) -> Self {
        Self {
            core: Rc::new(RefCell::new(core)),
            index: 0,
        }
    }

    pub fn get_index(&self) -> usize {
        self.index
    }

    pub fn increment_index(&mut self) {
        self.index += 1;
    }

    pub fn current_target_value(&self) -> usize {
        (*self.core).borrow().current_target_value()
    }

    pub fn reserve_last_target_value(&mut self) {
        self.core.borrow_mut().reserve_last_target_value()
    }

    pub fn add_capture(&mut self, capture: TrackedCapture) {
        let offset = self.get_index();

        fn adjust_offset(offset: usize, capture: TrackedCapture) -> TrackedCapture {
            match capture {
                TrackedCapture::Struct { key, captures } => TrackedCapture::Struct {
                    key,
                    captures: captures
                        .into_iter()
                        .map(|capture| adjust_offset(offset, capture))
                        .collect(),
                },

                TrackedCapture::Text { key, range } => TrackedCapture::Text {
                    key,
                    range: (range.start + offset)..range.end,
                },
            }
        }

        let capture = adjust_offset(offset, capture);
        self.core.borrow_mut().add_capture(capture)
    }
}
