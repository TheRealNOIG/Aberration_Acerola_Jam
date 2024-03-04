use minifb::{Key, KeyRepeat, Window};

use crate::{
    renderer::{fill_buffer_rand, fill_buffer_rand_bw},
    state::RenderState,
};

pub fn handle_input(
    render_state: &mut RenderState,
    buffer: &mut Vec<u32>,
    window: &mut Window,
) {
    if window.is_key_pressed(Key::Enter, KeyRepeat::No) {
        render_state.color_state = !render_state.color_state;
        if !render_state.debug_render {
            if render_state.color_state {
                fill_buffer_rand_bw(buffer);
            } else {
                fill_buffer_rand(buffer);
            }
        }
    }
    if window.is_key_pressed(Key::Space, KeyRepeat::No) {
        render_state.debug_render = !render_state.debug_render;
        if render_state.color_state {
            fill_buffer_rand_bw(buffer);
        } else {
            fill_buffer_rand(buffer);
        }
    }
}
