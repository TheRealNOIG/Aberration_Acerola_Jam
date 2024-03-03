use std::time::{Duration, Instant};

pub struct Timer {
    tick: usize,
    elapsed: Instant,
    duration: Duration,
    callback: Box<dyn Fn(usize) > ,
}

impl Timer {
    pub fn new(duration_milli_seconds: u64, callback: Box<dyn Fn(usize) >) -> Timer {
        Timer {
            tick: 0,
            elapsed: Instant::now(),
            duration: Duration::from_millis(duration_milli_seconds),
            callback,
        }
    }

    pub fn tick(&mut self) {
        self.tick += 1;
        let now = Instant::now();

        if now.duration_since(self.elapsed) >= self.duration {
            (self.callback)(self.tick);
            self.tick = 0;
            self.elapsed = now;
        }
    }
}



// let mut last_fps_time = Instant::now();
// let mut frame_count = 0;
// frame_count += 1;
// let elapsed = last_fps_time.elapsed();
// if elapsed >= Duration::from_secs(1) {
//     let fps = frame_count;
//     window.set_title(&format!("FPS: {}", fps));
//     frame_count = 0;
//     last_fps_time = Instant::now();
// }
