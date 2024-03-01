mod renderer;

use std::{time::{Duration, Instant}, usize};
use minifb::{Key, Window, WindowOptions};
use renderer::scale_buffer;  

const WINDOW_WIDTH: usize = 1080;
const WINDOW_HEIGHT: usize = 720;
const BUFFER_WIDTH: usize = 4;
const BUFFER_HEIGHT: usize = 4;

fn main() {
    let mut buffer: Vec<u32> = vec![0; BUFFER_WIDTH * BUFFER_HEIGHT];
    let mut window = Window::new(
        "Test - ESC to exit",
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut last_fps_time = Instant::now();
    let mut frame_count = 0;

    let mut count = 0;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        for i in 0..buffer.len() {
            buffer[i] = 0;
        }

        buffer[count] = u32::max_value();
        count += 1;
        count = count % buffer.len();

        let scaled_buffer = scale_buffer(
            &buffer,
            BUFFER_WIDTH,
            BUFFER_HEIGHT,
            WINDOW_WIDTH,
            WINDOW_HEIGHT,
        );
        window
            .update_with_buffer(&scaled_buffer, WINDOW_WIDTH, WINDOW_HEIGHT)
            .unwrap();
        frame_count += 1;
        let elapsed = last_fps_time.elapsed();
        if elapsed >= Duration::from_secs(1) {
            let fps = frame_count;
            window.set_title(&format!("FPS: {}", fps));
            frame_count = 0;
            last_fps_time = Instant::now();
        }
    }
}

