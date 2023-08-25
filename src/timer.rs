use std::{thread, time::{Instant, Duration}};

pub struct Timer {
    timestamp_start: Instant,
    timestamp_end: Instant,
    time_between: Duration,
}
impl Timer {
    pub fn new() -> Timer {
        Timer {
            timestamp_start: Instant::now(),
            timestamp_end: Instant::now(),
            time_between: Duration::new(0, 0),
        }
    }
    pub fn start(&mut self) {
        self.timestamp_start = Instant::now();
    }
    pub fn stop(&mut self) {
        self.timestamp_end = Instant::now();
        self.time_between = self.timestamp_end.duration_since(self.timestamp_start);
    }
    pub fn get_time_between(&self) -> Duration {
        return self.time_between;
    }
    pub fn get_time_between_as_micros(&self) -> u128 {
        return self.time_between.as_micros();
    }
    pub fn wait(&self, max: u128) {
        if max > self.time_between.as_micros() {
            thread::sleep(Duration::from_micros((max - self.time_between.as_micros()) as u64));
        } else {
            logger::log(
                logger::PREFIX_WARN,
                format!("To long update time: {}\\{}ms",
                    max,
                    self.time_between.as_micros(),
                ).as_str()
            );
        }
    }
}