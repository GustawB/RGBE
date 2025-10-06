use std::sync::{Mutex, MutexGuard};

pub struct Clock {
    counter: u64
}

impl Clock {
    pub fn new () -> Clock {
        Clock {
            counter: 0,
        }
    }

    pub fn increment(&mut self) {
        self.counter += 1;
    }
}