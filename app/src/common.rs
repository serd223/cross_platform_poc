use std::ops::{Add, Mul};

const WIDTH_BASE: usize = 16;
const HEIGHT_BASE: usize = 9;

const CANVAS_SIZE_FACTOR: usize = 20;
pub const CANVAS_WIDTH: usize = WIDTH_BASE * CANVAS_SIZE_FACTOR;
pub const CANVAS_HEIGHT: usize = HEIGHT_BASE * CANVAS_SIZE_FACTOR;

/// Up    = 0
/// Down  = 1
/// Left  = 2
/// Right = 3
/// Pause = 4
/// Restart = 5
#[repr(usize)]
#[derive(Clone, Copy)]
pub enum Control {
    Up = 0,
    Down,
    Left,
    Right,
    Pause,
    Restart,
    MouseLeft,
}

impl Control {
    pub const COUNT: usize = 7;
}

pub trait PlatformColor {
    fn from_rgbau32(rgba: u32) -> u32;
}
#[derive(Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
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

#[derive(Clone, Copy)]
pub struct Rgba(pub u32);
impl framebrush::Color<u32> for Rgba {
    fn pixel(&self, _buf: &mut [u32], _index: usize) -> u32 {
        self.0
    }
}
