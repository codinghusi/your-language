use std::ops::RangeFrom;

pub struct IdGen {
    iter: RangeFrom<usize>
}

impl IdGen {
    pub fn new() -> Self {
        Self {
            iter: (0..).into_iter()
        }
    }

    pub fn next(&mut self) -> usize {
        self.iter.next().unwrap()
    }
}