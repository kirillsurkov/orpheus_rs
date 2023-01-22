pub struct Stopwatch {
    last_time: std::time::SystemTime,
}

impl Stopwatch {
    pub fn new() -> Self {
        Self {
            last_time: std::time::SystemTime::now(),
        }
    }

    pub fn split(&mut self) -> f32 {
        let now = std::time::SystemTime::now();
        let delta = now.duration_since(self.last_time).unwrap().as_secs_f32();
        self.last_time = now;
        return delta;
    }
}