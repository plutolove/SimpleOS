#![no_std] // 不链接Rust标准库
#![no_main] // 禁用所有Rust层级的入口点

mod vga;

use vga::{Writer, Color, genColorCode, Buffer};
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut w = Writer::new(Color::Green, Color::Black);
    w.write_string("Hello World!\n");
    w.write_string("sdfgwrdfgdfghdfgh");
    loop {}
}