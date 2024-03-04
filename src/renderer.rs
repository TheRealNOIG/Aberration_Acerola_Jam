use std::{
    cmp::{max, min},
    f32::consts::PI,
    u32,
};

use rand::Rng;

use crate::{
    player::Player, raycast::ray_march, BUFFER_HEIGHT, BUFFER_WIDTH, FOV, MAP, TWO_PI,
    WINDOW_HEIGHT, WINDOW_WIDTH,
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
pub fn fill_buffer_rand_bw(buffer: &mut Vec<u32>) {
    let mut rng = rand::thread_rng();

    for i in 0..buffer.len() {
        let color_value = if rng.gen::<bool>() { 255 } else { 0 };
        let color = ((color_value as u32) << 16) | ((color_value as u32) << 8) | (color_value as u32);
        buffer[i] = color;
    }
}

pub fn render_black_and_white(buffer: &mut Vec<u32>, player: &Player) {
    for i in 0..buffer.len() {
        buffer[i] = 0;
    }

    render_aberration(buffer, player)
}

pub fn render_aberration(prev_buffer: &mut Vec<u32>, player: &Player) {
    let mut buffer: Vec<u32> = vec![0; BUFFER_WIDTH * BUFFER_HEIGHT];

    for x in 0..BUFFER_WIDTH {
        let ray_angle = (player.rotation - FOV / 2.0) + (x as f32) * (FOV / BUFFER_WIDTH as f32);
        let (ray, uv) = ray_march(player.pos_x, player.pos_y, ray_angle, 32, 32, &MAP);
        let height = ((BUFFER_HEIGHT as f32) / ray * 2.00) as usize;

        let wall_start = BUFFER_HEIGHT.saturating_sub(height) / 2;
        let wall_end = wall_start + height;

        for y in wall_start..min(wall_end, BUFFER_HEIGHT) {
            if y == wall_start || y == min(wall_end, BUFFER_HEIGHT) - 1 {
                buffer[x + y * BUFFER_WIDTH] = 0xFF000000;
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
}

pub fn draw_portal(buffer: &mut Vec<u32>, player: &Player) {
    // TODO make a real sprite renderer
    let half_fov = FOV / 2.0;

    let test: Player = Player::new(16.0, 16.0, 0.0);
    let delta_x = test.pos_x - player.pos_x;
    let delta_y = test.pos_y - player.pos_y;

    // Calculate the distance and angle to the sprite
    let distance_to_sprite = (delta_x.powi(2) + delta_y.powi(2)).sqrt();
    let sprite_angle = delta_y.atan2(delta_x);

    // Translate the sprite's angle to FOV coordinates
    let sprite_angle_relative_to_fov = (sprite_angle - player.rotation).rem_euclid(TWO_PI);

    let sprite_screen_x = (((sprite_angle_relative_to_fov + half_fov) / FOV) * BUFFER_WIDTH as f32)
        % BUFFER_WIDTH as f32;

    if sprite_angle_relative_to_fov <= half_fov
        || sprite_angle_relative_to_fov >= (TWO_PI) - half_fov
    {
        let sprite_size = BUFFER_HEIGHT as f32 / distance_to_sprite;
        let sprite_screen_y = (BUFFER_HEIGHT as f32 - sprite_size / 2.0) / 2.0;
        let portal_width = sprite_size * 1.25;
        let portal_height = sprite_size * 2.0;

        for y in 0..BUFFER_HEIGHT {
            for x in 0..BUFFER_WIDTH {
                let dx = x as f32 - sprite_screen_x;
                let dy = y as f32 - sprite_screen_y;
                if (dx.powi(2) / portal_width.powi(2)) + (dy.powi(2) / portal_height.powi(2)) <= 1.0
                {
                    let i = x + y * BUFFER_WIDTH;

                    // this one dose a cool cleanup effect of the static
                    // This should be used when the portal is not being blocked by a wall
                    // buffer[i] = 0xFF000000;

                    // buffer[i] = buffer[i] ^ 0xFF000000;

                    // and this one used when behind a wall.
                    let mut rng = rand::thread_rng();
                    if rng.gen_range(0..=100) < 10 {
                        buffer[i] = buffer[i] ^ 0xFFFFFFFF;
                    }
                }
            }
        }
    }
}
