use std::time::{Duration, Instant};

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Timing {
    pub start_of_previous_frame: Instant,
    pub start_of_current_frame: Instant,
    pub length_of_previous_frame: Duration,
}

impl Timing {
    pub fn new() -> Self {
        let start_of_previous_frame = Instant::now();
        let start_of_current_frame = Instant::now();
        let length_of_previous_frame =
            start_of_current_frame.duration_since(start_of_previous_frame);
        Timing {
            start_of_previous_frame,
            start_of_current_frame,
            length_of_previous_frame,
        }
    }

    pub fn update(&mut self) {
        self.start_of_previous_frame = self.start_of_current_frame;
        self.start_of_current_frame = Instant::now();
        self.length_of_previous_frame = self
            .start_of_current_frame
            .duration_since(self.start_of_previous_frame);
    }
}