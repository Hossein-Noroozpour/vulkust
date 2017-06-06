use std::sync::{Mutex, Condvar};

pub struct Semaphore {
    lock: Mutex<isize>,
    cvar: Condvar,
}

// NOTE: This is implementation of weak version of semaphore
impl Semaphore {
    pub fn new(count: isize) -> Semaphore {
        Semaphore {
            lock: Mutex::new(count),
            cvar: Condvar::new(),
        }
    }

    pub fn acquire(&self) {
        let mut count = self.lock.lock().unwrap();
        while *count <= 0 {
            count = self.cvar.wait(count).unwrap();
        }
        *count -= 1;
    }

    pub fn release(&self) {
        *self.lock.lock().unwrap() += 1;
        self.cvar.notify_one();
    }
}
