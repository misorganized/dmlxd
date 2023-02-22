use std::time::{Duration, Instant};

pub struct Timer {
    start_time: Instant,
    elapsed: Duration,
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            start_time: Instant::now(),
            elapsed: Duration::default(),
        }
    }

    pub fn pointer(&mut self, name: &str) {
        self.elapsed = self.start_time.elapsed();
        println!("{} took: {}ms", name, self.elapsed.as_millis());
    }
}