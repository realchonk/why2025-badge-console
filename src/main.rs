#![no_std]
#![no_main]

use core::{ffi::{c_char, c_int}, panic::PanicInfo};

mod badgevms;

#[unsafe(no_mangle)]
pub extern "C" fn main(_argc: c_int, _argv: *mut *mut c_char) -> i32 {
    println!("Hello World from Rust!");
    0
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("PANIC: {info}");
    unsafe { why2025_badge_sys::abort() };
}
