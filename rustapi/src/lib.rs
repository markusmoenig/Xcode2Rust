mod disc;

use disc::Disc;

use lazy_static::lazy_static; // 1.4.0
use std::sync::Mutex;

lazy_static! {
    static ref DISC: Mutex<Disc> = Mutex::new(Disc::new());
}

#[no_mangle]
pub extern "C" fn rust_draw(pixels: *mut u8, width: u32, height: u32) {
    let length = width as usize * height as usize * 4;
    let slice = unsafe { std::slice::from_raw_parts_mut(pixels, length) };

    DISC.lock().unwrap().draw(slice, width as usize, height as usize);
}

#[no_mangle]
pub extern "C" fn rust_target_fps() -> u32 {
    0
}

#[no_mangle]
pub extern "C" fn rust_touch_down(x: f32, y: f32) -> bool {
    println!("touch down {} {}", x, y);
    DISC.lock().unwrap().touch_down(x, y)
}

#[no_mangle]
pub extern "C" fn rust_touch_dragged(x: f32, y: f32) -> bool {
    println!("touch dragged {} {}", x, y);
    DISC.lock().unwrap().touch_dragged(x, y)
}

#[no_mangle]
pub extern "C" fn rust_touch_up(x: f32, y: f32) -> bool {
    println!("touch up {} {}", x, y);
    DISC.lock().unwrap().touch_up(x, y)
}