mod sphere;

use sphere::Sphere;

use lazy_static::lazy_static; // 1.4.0
use std::sync::Mutex;

lazy_static! {
    static ref SPHERE: Mutex<Sphere> = Mutex::new(Sphere::new());
}

#[no_mangle]
pub extern "C" fn rust_draw(pixels: *mut u8, width: u32, height: u32) {
    let length = width as usize * height as usize * 4;
    let slice = unsafe { std::slice::from_raw_parts_mut(pixels, length) };

    SPHERE.lock().unwrap().draw(slice, width as usize, height as usize);
}

#[no_mangle]
pub extern "C" fn rust_mouse_down(x: u32, y: u32) -> bool {
    println!("mouse down {} {}", x, y);
    true
}

#[no_mangle]
pub extern "C" fn rust_mouse_dragged(x: u32, y: u32) -> bool {
    println!("mouse dragged {} {}", x, y);
    true
}

#[no_mangle]
pub extern "C" fn rust_mouse_up(x: u32, y: u32) -> bool {
    println!("mouse up {} {}", x, y);
    true
}