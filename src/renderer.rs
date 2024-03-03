use std::{cmp::min, u32};

use rand::Rng;

use crate::{
    player::Player, raycast::ray_march, BUFFER_HEIGHT, BUFFER_WIDTH, FOV, MAP, WINDOW_HEIGHT,
    WINDOW_WIDTH,
};

pub fn scale_buffer(
    buffer: &[u32],
    src_width: usize,
    src_height: usize,
    dest_width: usize,
    dest_height: usize,
) -> Vec<u32> {
    let mut scaled_buffer = vec![0; dest_width * dest_height];

    let x_ratio = (src_width << 16) / dest_width;
    let y_ratio = (src_height << 16) / dest_height;

    for y in 0..dest_height {
        for x in 0..dest_width {
            let src_x = (x * x_ratio) >> 16;
            let src_y = (y * y_ratio) >> 16;

            let src_index = src_y * src_width + src_x;

            scaled_buffer[y * dest_width + x] = buffer[src_index];
        }
    }

    scaled_buffer
}

pub fn fill_buffer_rand(buffer: &mut Vec<u32>) {
    let mut rng = rand::thread_rng();

    for i in 0..buffer.len() {
        let red = rng.gen::<u8>();
        let green = rng.gen::<u8>();
        let blue = rng.gen::<u8>();
        let color = ((red as u32) << 16) | ((green as u32) << 8) | (blue as u32);
        buffer[i] = color;
    }
}

pub fn render_black_and_white(buffer: &mut Vec<u32>, player: &Player) -> Vec<u32> {
    for i in 0..buffer.len() {
        buffer[i] = 0;
    }

    for x in 0..BUFFER_WIDTH {
        let ray_angle = (player.rotation - FOV / 2.0) + (x as f32) * (FOV / BUFFER_WIDTH as f32);
        let (ray, uv) = ray_march(player.pos_x, player.pos_y, ray_angle, 32, 32, &MAP);
        let height = ((BUFFER_HEIGHT as f32) / ray * 2.00) as usize;

        let wall_start = BUFFER_HEIGHT.saturating_sub(height) / 2;
        let wall_end = wall_start + height;

        for y in wall_start..min(wall_end, BUFFER_HEIGHT) {
            if y == wall_start || y == min(wall_end, BUFFER_HEIGHT) - 1 {
                buffer[x + y * BUFFER_WIDTH] = buffer[x + y * BUFFER_WIDTH] ^ 0x00FFFFFF;
            }
            if uv < 0.03 || uv > 0.99 {
                buffer[x + y * BUFFER_WIDTH] = buffer[x + y * BUFFER_WIDTH] ^ 0x00FFFFFF;
            }
        }
    }

    scale_buffer(
        &buffer,
        BUFFER_WIDTH,
        BUFFER_HEIGHT,
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
    )
}

pub fn render_aberration(prev_buffer: &mut Vec<u32>, player: &Player) -> Vec<u32> {
    let mut buffer: Vec<u32> = vec![0; BUFFER_WIDTH * BUFFER_HEIGHT];

    for x in 0..BUFFER_WIDTH {
        let ray_angle = (player.rotation - FOV / 2.0) + (x as f32) * (FOV / BUFFER_WIDTH as f32);
        let (ray, uv) = ray_march(player.pos_x, player.pos_y, ray_angle, 32, 32, &MAP);
        let height = ((BUFFER_HEIGHT as f32) / ray * 2.00) as usize;

        let wall_start = BUFFER_HEIGHT.saturating_sub(height) / 2;
        let wall_end = wall_start + height;

        for y in wall_start..min(wall_end, BUFFER_HEIGHT) {
            if y == wall_start || y == min(wall_end, BUFFER_HEIGHT) - 1 {
                buffer[x + y * BUFFER_WIDTH] =  0xFF000000;
            }
            if uv < 0.03 || uv > 0.99 {
                buffer[x + y * BUFFER_WIDTH] = 0xFF000000;
            }
        }
    }
    

    for i in 0..buffer.len() {
        if prev_buffer[i] & 0xFF000000 != buffer[i] & 0xFF000000 {
            prev_buffer[i] = prev_buffer[i] ^ 0xFFFFFFFF;
        }
    }

    scale_buffer(
        &prev_buffer,
        BUFFER_WIDTH,
        BUFFER_HEIGHT,
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
    )
}
