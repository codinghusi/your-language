use std::ops::Range;

pub enum TrackedCapture {
    Struct { key: String, captures: Vec<Self> },
    Text { key: String, range: Range<usize> },
}

pub struct TrackingCore {
    pub target_value: usize,
    pub captures: Vec<TrackedCapture>,
}

impl TrackingCore {
    pub fn new() -> Self {
        Self {
            target_value: 0,
            captures: vec![],
        }
    }

    pub fn current_target_value(&self) -> usize {
        self.target_value
    }

    pub fn reserve_last_target_value(&mut self) {
        self.target_value += 1;
    }

    pub fn add_capture(&mut self, capture: TrackedCapture) {
        self.captures.push(capture)
    }
}
