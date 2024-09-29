use game::{Game, GameColor, HEIGHT, WIDTH};
use minifb::{Window, WindowOptions};

struct ARgbColor {}

impl GameColor for ARgbColor {
    fn from_rgbau32(rgba: u32) -> u32 {
        let a = rgba & 0x000000ff;
        let ret = rgba >> 8;
        let ret = ret | (a << 24);
        ret
    }
}

pub fn main() {
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

    window.set_target_fps(60);
    while window.is_open() {
        {
            let size = window.get_size();
            if !((width, height) == size) {
                (width, height) = size;
                buffer.resize(size.0 * size.1, 0);
            }
        }
        game.frame::<ARgbColor>(buffer.as_mut_slice(), width, height, 0.);

        window.update_with_buffer(&buffer, width, height).unwrap();
    }
}
