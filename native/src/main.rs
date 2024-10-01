use std::{collections::HashMap, time::Instant};

use game::{Controls, Game, GameColor};
use minifb::{Key, Window, WindowOptions};

struct ARgbColor {}

impl GameColor for ARgbColor {
    fn from_rgbau32(rgba: u32) -> u32 {
        let a = rgba & 0x000000ff;
        let ret = rgba >> 8;
        let ret = ret | (a << 24);
        ret
    }
}

const WIDTH: usize = 640;
const HEIGHT: usize = 360;
pub fn main() {
    let controls = HashMap::from([
        (Key::W, Controls::SnakeUp),
        (Key::A, Controls::SnakeLeft),
        (Key::S, Controls::SnakeDown),
        (Key::D, Controls::SnakeRight),
        (Key::Escape, Controls::Pause),
    ]);
    let (mut width, mut height) = (WIDTH, HEIGHT);
    let mut buffer: Vec<u32> = vec![0; width * height];
    let mut game = Game::default();
    let mut window = Window::new(
        "Snake",
        width,
        height,
        WindowOptions {
            resize: true,
            ..Default::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });
    let keys_down = &mut [false; Controls::COUNT];

    let mut prev_time = Instant::now();
    window.set_target_fps(144);
    while window.is_open() {
        let now = Instant::now();
        let delta = now.duration_since(prev_time).as_secs_f32();
        prev_time = now;
        for (&key, &control) in controls.iter() {
            if window.is_key_down(key) {
                keys_down[control as usize] = true;
            }
            if window.is_key_released(key) {
                keys_down[control as usize] = false;
            }
        }
        {
            let size = window.get_size();
            if (width, height) != size {
                (width, height) = size;
                // If the user decreases the size of the window, keep the buffer's capacity the same.
                // This way we don't need to allocate memory if the user increases the size of the window but it's still smaller than our buffer.
                if buffer.len() < size.0 * size.1 {
                    buffer.resize(size.0 * size.1, 0);
                }
            }
        }
        game::frame::<ARgbColor>(
            &mut game,
            buffer.as_mut_slice(),
            width,
            height,
            delta,
            keys_down,
        );

        window.update_with_buffer(&buffer, width, height).unwrap();
    }
}
