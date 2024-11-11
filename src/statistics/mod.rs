use crate::prelude::*;

pub struct Statistics {
    time: time::Duration,
    frame_count: u64,
}

impl Statistics {
    pub fn init() -> Self {
        Self {
            time: time::Duration::from_secs(0),
            frame_count: 0,
        }
    }
    pub fn collect(self: &mut Self, time_delta: time::Duration) {
        self.time += time_delta;
        self.frame_count += 1;

        while self.time >= time::Duration::from_secs(1) {
            self.time -= time::Duration::from_secs(1);
            self.frame_count = 0;
        }
    }


}
