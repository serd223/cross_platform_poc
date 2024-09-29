use framebrush::{Canvas, Color};

pub const WIDTH: usize = 640;
pub const HEIGHT: usize = 360;

pub trait GameColor {
    fn from_rgbau32(rgba: u32) -> u32;
}
pub struct Game {
    pub controls_down: [bool; Controls::COUNT],
    pub controls_pressed: [bool; Controls::COUNT],
}

impl Default for Game {
    fn default() -> Self {
        Self::const_default()
    }
}

pub enum Controls {
    SnakeUp,
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

impl Game {
    pub const fn const_default() -> Self {
        Self {
            controls_down: [false; Controls::COUNT],
            controls_pressed: [false; Controls::COUNT],
        }
    }

    /// Do not construct `Rgba` directly with raw `rgba` data, instead use `C::from_rgbau32` to convert your data to the correct format.
    /// That is because different platforms have different pixel formats (`minifb` expects `argb` while the html canvas expects `abgr`).
    /// `C::from_rgbau32` returns data in the platforms expected pixel format. So, doing bitwise or arithmetical operations on
    /// returned data will most likely result in bugs.
    pub fn frame<C: GameColor>(
        &mut self,
        buf: &mut [u32],
        width: usize,
        height: usize,
        _delta: f32,
    ) {
        let red = Rgba(C::from_rgbau32(0xff0000ff));
        let bg = Rgba(C::from_rgbau32(0x282c34ff));

        let mut canvas = Canvas::new(buf, (width, height), (320, 180));
        canvas.clear(bg);
        canvas.rect(10, 20, 50, 20, &red);
    }
}
