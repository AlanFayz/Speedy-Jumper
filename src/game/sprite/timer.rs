use macroquad::time::get_time;
use std::time::Duration;

pub struct Timer {
    start: Duration,
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            start: Duration::from_secs_f64(get_time())
        }
    }

    pub fn has_elapsed(&self, duration: Duration) -> bool {
        self.elapsed() >= duration
    }

    pub fn elapsed(&self) -> Duration {
        Duration::from_secs_f64(get_time()) - self.start
    }
    
    pub fn reset(&mut self) {
        self.start = Duration::from_secs_f64(get_time());
    }
}

