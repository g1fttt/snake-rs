use std::time::{Duration, Instant};

pub struct Timer {
    start: Instant,
    delta: Option<Duration>,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            start: Instant::now(),
            delta: None,
        }
    }
}

impl Timer {
    pub fn tick(&mut self) {
        self.delta = Some(Instant::now().duration_since(self.start));
    }

    pub fn delta(&self) -> Option<Duration> {
        self.delta
    }

    pub fn reset(&mut self) {
        self.start = Instant::now();
        self.delta = None;
    }
}
