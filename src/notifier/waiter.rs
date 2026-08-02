use std::task::Waker;


pub struct Waiter {
    waker: Option<Waker>,
    ready: bool
}

impl Waiter {
    pub fn new() -> Self {
        Self {
            waker: None,
            ready: false
        }
    }

    pub fn set_ready_to_retry(&mut self) {
        self.ready = true
    }

    pub fn set_waiting_to_retry(&mut self) {
        self.ready = false
    }

    pub fn wake(&mut self) {
        if let Some(waker) = self.waker.take() {
            waker.wake();
        }
    }

    pub fn set_waker(&mut self, waker: Waker) {
        self.waker.replace(waker).expect("Expected no Waker");
    }

    pub fn is_ready_to_retry(&self) -> bool {
        self.ready
    }
}