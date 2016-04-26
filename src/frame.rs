
pub struct Frame {
    data_stack: Vec<i64>,
    // block_stack: Vec<i64>,
}

impl Frame {
    pub fn new() -> Frame {
        Frame {
            data_stack: Vec::new(),
        }
    }

    pub fn push(&mut self, datum: i64) {
        self.data_stack.push(datum);
    }

    pub fn pop(&mut self) -> Option<i64> {
        return self.data_stack.pop();
    }
}
