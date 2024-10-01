use std::ops::{Add, Mul};

use framebrush::{Canvas, Color};

const CANVAS_WIDTH: usize = 320;
const CANVAS_HEIGHT: usize = 180;

/// Up    = 0
/// Down  = 1
/// Left  = 2
/// Right = 3
/// Pause = 4
#[repr(usize)]
#[derive(Clone, Copy)]
pub enum Controls {
    SnakeUp = 0,
    SnakeDown,
    SnakeLeft,
    SnakeRight,
    Pause,
}

impl Controls {
    pub const COUNT: usize = 5;
}
pub trait GameColor {
    fn from_rgbau32(rgba: u32) -> u32;
}

pub struct Game {
    pub snake: Snake,
}
impl Default for Game {
    fn default() -> Self {
        Self {
            snake: Snake::default(),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Vec2 {
    x: f32,
    y: f32,
}

impl Mul<f32> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Vec2) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

pub struct Vec2I32 {
    x: i32,
    y: i32,
}

pub struct Snake {
    body: Vec<Vec2I32>,
    head: Vec2,
    speed: f32,
}

impl Snake {
    pub fn update(&mut self, delta: f32, dir: Vec2) {
        self.head = self.head + dir * (self.speed * delta);
    }
}

impl Default for Snake {
    fn default() -> Self {
        Self {
            body: vec![Vec2I32 { x: 0, y: 0 }],
            head: Vec2 { x: 0., y: 0. },
            speed: 150.,
        }
    }
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
    delta: f32,
    keys_down: &[bool],
) {
    let mut snake_color = Rgba(C::from_rgbau32(0x000000ff));
    let bg = Rgba(C::from_rgbau32(0x282c34ff));

    let mut snake_dir = Vec2 { x: 0., y: 0. };
    if keys_down[Controls::SnakeUp as usize] {
        snake_color = Rgba(C::from_rgbau32(0xff0000ff));
        snake_dir.y -= 1.;
    }
    if keys_down[Controls::SnakeDown as usize] {
        snake_color = Rgba(C::from_rgbau32(0x00ff00ff));
        snake_dir.y += 1.;
    }
    if keys_down[Controls::SnakeLeft as usize] {
        snake_color = Rgba(C::from_rgbau32(0x0000ffff));
        snake_dir.x -= 1.;
    }
    if keys_down[Controls::SnakeRight as usize] {
        snake_color = Rgba(C::from_rgbau32(0xffffffff));
        snake_dir.x += 1.;
    }

    g.snake.update(delta, snake_dir);

    let mut canvas = Canvas::new(buf, (width, height), (CANVAS_WIDTH, CANVAS_HEIGHT));
    canvas.clear(bg);
    canvas.rect(
        g.snake.head.x as i32,
        g.snake.head.y as i32,
        50,
        20,
        &snake_color,
    );
}
