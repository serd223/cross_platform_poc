use framebrush::Canvas;
mod common;
pub use common::*;

pub struct App {
    square: Square,
    prev_keys_down: Vec<bool>,
    paused: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            square: Square::default(),
            prev_keys_down: vec![false; Control::COUNT],
            paused: false,
        }
    }
}

pub struct Square {
    pos: Vec2,
    speed: f32,
}

impl Default for Square {
    fn default() -> Self {
        Self {
            pos: Vec2 { x: 0., y: 0. },
            speed: 150.,
        }
    }
}

impl Square {
    pub fn update(&mut self, delta: f32, dir: Vec2) {
        self.pos = self.pos + dir * (self.speed * delta);
    }
}

/// Do not construct `Rgba` directly with raw `rgba` data, instead use `C::from_rgbau32` to convert your data to the correct format.
/// That is because different platforms have different pixel formats (`minifb` expects `argb` while the html canvas expects `abgr`).
/// `C::from_rgbau32` returns data in the platforms expected pixel format. So, doing bitwise or arithmetical operations on
/// returned data will most likely result in bugs.
pub fn frame<C: GameColor>(
    g: &mut App,
    buf: &mut [u32],
    width: usize,
    height: usize,
    delta: f32,
    keys_down: &[bool],
) {
    let bg = Rgba(C::from_rgbau32(0x282c34ff));

    let mut dir = Vec2 { x: 0., y: 0. };
    if g.prev_keys_down[Control::Pause as usize] && !keys_down[Control::Pause as usize] {
        g.paused = !g.paused;
    }
    if keys_down[Control::Up as usize] {
        dir.y -= 1.;
    }
    if keys_down[Control::Down as usize] {
        dir.y += 1.;
    }
    if keys_down[Control::Left as usize] {
        dir.x -= 1.;
    }
    if keys_down[Control::Right as usize] {
        dir.x += 1.;
    }
    if keys_down[Control::Restart as usize] {
        g.square = Square::default();
    }
    g.prev_keys_down.copy_from_slice(keys_down);

    if g.paused {
        dir.x = 0.;
        dir.y = 0.;
    }

    if !g.paused {
        g.square.update(delta, dir);
    }
    let color = if g.paused {
        Rgba(C::from_rgbau32(0xff0000ff))
    } else {
        Rgba(C::from_rgbau32(0x00ff00ff))
    };

    let mut canvas = Canvas::new(buf, (width, height), (CANVAS_WIDTH, CANVAS_HEIGHT));
    canvas.clear(bg);
    canvas.rect(g.square.pos.x as i32, g.square.pos.y as i32, 20, 20, &color);
}
