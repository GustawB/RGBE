use std::sync::{Mutex, MutexGuard};

pub struct Clock {
    counter: Mutex<u64>
}

impl Clock {
    pub fn new () -> Clock {
        Clock {
            counter: Mutex::new(0),
        }
    }

    fn increment(&mut self) {
        let mut locked_counter: MutexGuard<'_, u64> = self.counter.lock().unwrap();
        *locked_counter += 1;
    }

    pub fn mcycle(&mut self) {
        for _ in 0..4 {
            self.increment();
        }
    }
}