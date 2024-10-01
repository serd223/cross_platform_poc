use std::{mem, ptr::slice_from_raw_parts_mut};

use game::{Controls, Game, GameColor};

struct ABgrColor {}

impl GameColor for ABgrColor {
    fn from_rgbau32(rgba: u32) -> u32 {
        let r = rgba & 0xff000000;
        let g = rgba & 0x00ff0000;
        let b = rgba & 0x0000ff00;
        let a = rgba & 0x000000ff;
        (a << 24) | (b << 8) | (g >> 8) | (r >> 24)
    }
}

#[no_mangle]
pub extern "C" fn allocate_image(width: usize, height: usize) -> *mut u32 {
    let mut v: Vec<u32> = Vec::with_capacity(width * height);
    let ret = v.as_mut_ptr();
    mem::forget(v);
    ret
}

#[no_mangle]
pub extern "C" fn allocate_controls() -> *mut bool {
    let mut v: Vec<bool> = Vec::with_capacity(Controls::COUNT);
    let ret = v.as_mut_ptr();
    mem::forget(v);
    ret
}

#[no_mangle]
pub extern "C" fn allocate_game() -> *mut Game {
    let mut g = Game::default();
    let ret: *mut Game = &mut g;
    mem::forget(g);
    ret
}

#[no_mangle]
pub extern "C" fn frame(
    g_ptr: *mut Game,
    image_ptr: *mut u32,
    width: usize,
    height: usize,
    delta: f32,
    keys_down_ptr: *mut u8,
) {
    let image_data = slice_from_raw_parts_mut(image_ptr, width * height);
    let keys_down = slice_from_raw_parts_mut(keys_down_ptr, Controls::COUNT);
    let keys_down: Vec<bool> = unsafe {
        (&(*keys_down))
            .iter()
            .map(|&n| if n > 0 { true } else { false })
            .collect()
    };
    let g = unsafe { &mut (*g_ptr) };
    unsafe {
        game::frame::<ABgrColor>(
            g,
            &mut (*image_data),
            width,
            height,
            delta,
            keys_down.as_slice(),
        );
    }
}
