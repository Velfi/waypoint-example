use std::time::{Duration, Instant};

pub struct GameTimer {
    previous_instant: Instant,
    frame_time: f64,
    tick_counter: usize,
}

impl GameTimer {
    pub fn new() -> GameTimer {
        GameTimer {
            previous_instant: Instant::now(),
            frame_time: 0.0,
            tick_counter: 0,
        }
    }

    pub fn get_frame_time(&self) -> f64 {
        self.frame_time
    }

    pub fn get_ticks(&self) -> usize {
        self.tick_counter
    }

    pub fn tick(&mut self) {
        self.frame_time = duration_to_f64(self.previous_instant.elapsed());
        self.previous_instant = Instant::now();
        self.tick_counter += 1;
    }
}

impl Default for GameTimer {
    fn default() -> Self {
        Self::new()
    }
}

fn duration_to_f64(duration: Duration) -> f64 {
    duration.as_secs() as f64 + f64::from(duration.subsec_nanos()) * 1e-9
}
