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

struct Square {
    pos: Vec2,
    size: usize,
    speed: f32,
}

impl Default for Square {
    fn default() -> Self {
        Self {
            pos: Vec2 { x: 0., y: 0. },
            size: 20,
            speed: 150.,
        }
    }
}

impl Square {
    fn update(&mut self, delta: f32, dir: Vec2) {
        self.pos = self.pos + dir * (self.speed * delta);
    }
}

/// Do not construct `Rgba` directly with raw `rgba` data, instead use `C::from_rgbau32` to convert your data to the correct format.
/// That is because different platforms have different pixel formats (`minifb` expects `argb` while the html canvas expects `abgr`).
/// `C::from_rgbau32` returns data in the platforms expected pixel format. So, doing bitwise or arithmetical operations on
/// returned data will most likely result in bugs.
pub fn frame<C: PlatformColor>(
    app: &mut App,
    buf: &mut [u32],
    width: usize,
    height: usize,
    delta: f32,
    keys_down: &[bool],
    mouse_pos_x: f32,
    mouse_pos_y: f32,
) {
    let bg = Rgba(C::from_rgbau32(0x282c34ff));

    let mut dir = Vec2 { x: 0., y: 0. };
    if app.prev_keys_down[Control::Pause as usize] && !keys_down[Control::Pause as usize] {
        app.paused = !app.paused;
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
        app.square = Square::default();
    }
    if keys_down[Control::MouseLeft as usize] {
        app.square.pos.x = mouse_pos_x * CANVAS_WIDTH as f32 / width as f32;
        app.square.pos.y = mouse_pos_y * CANVAS_HEIGHT as f32 / height as f32;
    }
    app.prev_keys_down.copy_from_slice(keys_down);

    if app.paused {
        dir.x = 0.;
        dir.y = 0.;
    }

    if !app.paused {
        app.square.update(delta, dir);
    }

    let color = if app.paused {
        Rgba(C::from_rgbau32(0xff0000ff))
    } else {
        Rgba(C::from_rgbau32(0x00ff00ff))
    };

    let mut canvas = Canvas::new(buf, (width, height), (CANVAS_WIDTH, CANVAS_HEIGHT));
    canvas.clear(bg);
    canvas.rect(
        app.square.pos.x as i32,
        app.square.pos.y as i32,
        app.square.size,
        app.square.size,
        &color,
    );
}
