use std::f32::consts::PI;

use minifb::{Key, Window};

#[derive(Debug)]
pub struct Player {
    pub rotation: f32,
    pub pos_x: f32,
    pub pos_y: f32,
}
impl Player {
    pub fn new(x: f32, y: f32, rotation: f32) -> Player {
        Player {
            rotation,
            pos_x: x,
            pos_y: y,
        }
    }
}

pub fn move_player(player: &mut Player, window: &Window) {
    if window.is_key_down(Key::F) {
        player.pos_x += player.rotation.cos() * 0.1;
        player.pos_y += player.rotation.sin() * 0.1;
    }
    if window.is_key_down(Key::S) {
        player.pos_x -= player.rotation.cos() * 0.1;
        player.pos_y -= player.rotation.sin() * 0.1;
    }
    if window.is_key_down(Key::R) {
        player.pos_x += (player.rotation - PI / 2.0).cos() * 0.1;
        player.pos_y += (player.rotation - PI / 2.0).sin() * 0.1;
    }
    if window.is_key_down(Key::T) {
        player.pos_x += (player.rotation + PI / 2.0).cos() * 0.1;
        player.pos_y += (player.rotation + PI / 2.0).sin() * 0.1;
    }
    if window.is_key_down(Key::P) {
        player.rotation += 0.01;
        if player.rotation > 2.0 * PI {
            player.rotation -= 2.0 * PI;
        }
    }
    if window.is_key_down(Key::W) {
        player.rotation -= 0.01;
        if player.rotation < 0.0 {
            player.rotation += 2.0 * PI;
        }
    }
}
