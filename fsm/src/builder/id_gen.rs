
#[derive(Clone)]
pub struct IdGen {
    last_index: usize
}

impl IdGen {
    pub fn new() -> Self {
        Self {
            last_index: 0
        }
    }

    pub fn next(&mut self) -> usize {
        self.last_index += 1;
        self.last_index
    }

    pub fn last(&self) -> usize {
        self.last_index
    }
}