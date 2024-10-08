use std::{collections::HashMap, time::Instant};

use app::{App, Control, PlatformColor};
use minifb::{Key, Window, WindowOptions};

struct ARgbColor {}

impl PlatformColor for ARgbColor {
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
        (Key::W, Control::Up),
        (Key::A, Control::Left),
        (Key::S, Control::Down),
        (Key::D, Control::Right),
        (Key::Escape, Control::Pause),
        (Key::Space, Control::Restart),
    ]);
    let (mut width, mut height) = (WIDTH, HEIGHT);
    let mut buffer: Vec<u32> = vec![0; width * height];
    let mut app = App::default();
    let mut window = Window::new(
        "App",
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
    let keys_down = &mut [false; Control::COUNT];

    let mut prev_time = Instant::now();
    window.set_target_fps(144);
    while window.is_open() {
        let now = Instant::now();
        let delta = now.duration_since(prev_time).as_secs_f32();
        prev_time = now;

        // Resize buffer
        let (scrw, scrh) = window.get_size();
        if width * height != scrw * scrh {
            width = scrw;
            height = scrh;
            // If the user decreases the size of the window, keep the buffer's capacity the same.
            // This way we don't need to allocate memory if the user increases the size of the window but it's still smaller than our buffer.
            if buffer.len() < scrw * scrh {
                buffer.resize(scrw * scrh, 0);
            }
        }

        // Controls
        for (&key, &control) in controls.iter() {
            if window.is_key_down(key) {
                keys_down[control as usize] = true;
            }
            if window.is_key_released(key) {
                keys_down[control as usize] = false;
            }
        }
        keys_down[Control::MouseLeft as usize] = window.get_mouse_down(minifb::MouseButton::Left);
        let (mouse_x, mouse_y) = window
            .get_mouse_pos(minifb::MouseMode::Clamp)
            .unwrap_or((0., 0.));

        // Logic
        app::frame::<ARgbColor>(
            &mut app,
            buffer.as_mut_slice(),
            width,
            height,
            delta,
            keys_down,
            mouse_x,
            mouse_y,
        );

        // Render
        window.update_with_buffer(&buffer, width, height).unwrap();
    }
}
