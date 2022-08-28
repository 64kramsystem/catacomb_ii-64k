use std::sync::{Condvar, Mutex};

/// Synchronization functions which can time out return this value
/// if they time out.
const SDL_MUTEX_TIMEDOUT: i32 = 1;

/// This is the timeout value which corresponds to never time out.
const SDL_MUTEX_MAXWAIT: u32 = u32::MAX;

/// An implementation of semaphores using mutexes and condition variables
struct Semaphore {
    count: u32,
    waiters_count: u32,
    count_lock: Mutex<()>,
    count_nonzero: Condvar,
}

impl Semaphore {
    pub fn new(initial_value: u32) -> Self {
        Self {
            count: initial_value,
            waiters_count: 0,
            count_lock: Mutex::new(()),
            count_nonzero: Condvar::new(),
        }
    }

    pub fn try_wait(&mut self) -> i32 {
        let mut retval = SDL_MUTEX_TIMEDOUT;

        let _lock = self.count_lock.lock().unwrap();

        if self.count > 0 {
            self.count -= 1;
            retval = 0
        }

        retval
    }

    // pub fn wait(&self) -> i32 {
    //     self.wait_timeout(SDL_MUTEX_MAXWAIT)
    // }
}
