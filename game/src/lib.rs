use framebrush::{Canvas, Color};

const CANVAS_WIDTH: usize = 320;
const CANVAS_HEIGHT: usize = 180;

pub trait GameColor {
    fn from_rgbau32(rgba: u32) -> u32;
}

pub struct Game {
    pub controls_down: [bool; Controls::COUNT],
    pub controls_pressed: [bool; Controls::COUNT],
    pub snake: Snake,
}
impl Default for Game {
    fn default() -> Self {
        Self {
            controls_down: [false; Controls::COUNT],
            controls_pressed: [false; Controls::COUNT],
            snake: Snake::default(),
        }
    }
}

pub struct Vec2 {
    x: f32,
    y: f32,
}

pub struct Vec2I32 {
    x: i32,
    y: i32,
}

pub struct Snake {
    body: Vec<Vec2I32>,
    head: Vec2,
    direction: Direction,
}
impl Default for Snake {
    fn default() -> Self {
        Self {
            body: vec![Vec2I32 { x: 0, y: 0 }],
            head: Vec2 { x: 0., y: 0. },
            direction: Direction::Stopped,
        }
    }
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    Stopped,
}

#[repr(usize)]
pub enum Controls {
    SnakeUp = 0,
    SnakeDown,
    SnakeLeft,
    SnakeRight,
    Pause,
}

impl Controls {
    const COUNT: usize = 5;
}

pub struct Rgba(pub u32);
impl Color<u32> for Rgba {
    fn pixel(&self, _buf: &mut [u32], _index: usize) -> u32 {
        self.0
    }
}

/// Do not construct `Rgba` directly with raw `rgba` data, instead use `C::from_rgbau32` to convert your data to the correct format.
/// That is because different platforms have different pixel formats (`minifb` expects `argb` while the html canvas expects `abgr`).
/// `C::from_rgbau32` returns data in the platforms expected pixel format. So, doing bitwise or arithmetical operations on
/// returned data will most likely result in bugs.
pub fn frame<C: GameColor>(
    g: &mut Game,
    buf: &mut [u32],
    width: usize,
    height: usize,
    _delta: f32,
) {
    let red = Rgba(C::from_rgbau32(0xff0000ff));
    let bg = Rgba(C::from_rgbau32(0x282c34ff));
    let mut canvas = Canvas::new(buf, (width, height), (CANVAS_WIDTH, CANVAS_HEIGHT));
    canvas.clear(bg);
    canvas.rect(10, 20, 50, 20, &red);
}
