#![no_std] // 不链接Rust标准库
#![no_main] // 禁用所有Rust层级的入口点

mod vga;
mod spin;

use vga::{Writer, Color, Buffer};
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut w = Writer::new(Color::Cyan, Color::Black);

    w.write_string("Hello\nWorld!\n");
    //w.write_string("sdfgwrdfgdfghdfgh\n");

    for i in 65..91 {
        w.write_byte(i);
        w.write_string("\n");
    }

    //vga::WRITER.write_string("sdfgsrtdgfhdfghdfgh");
    unsafe {
        (vga::WRITER.borrow_mut()).write_string("sdfgerdsfgsdfgsdfg");
    }

    loop {}
}