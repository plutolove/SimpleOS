#![no_std] // 不链接Rust标准库
#![no_main] // 禁用所有Rust层级的入口点
#![feature(abi_x86_interrupt)]
#![feature(asm)]
#![feature(const_fn)]

pub mod interrupt;
pub mod spin;
pub mod vga;

use bootloader::BootInfo;

use core::fmt::Write;
use core::panic::PanicInfo;
use vga::{Color, Writer};
use interrupt::{init_idt, int3};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    /*
    let mut w = Writer::new(Color::Cyan, Color::Black);
    w.write_string("Hello\nWorld!\n");
    w.write_str("sdfgsdfg");
    for i in 65..91 {
        w.write_byte(i);
        w.write_string("\n");
    }
    vga::WRITER.lock().write_str("Hello again").unwrap();
    print!("sdfgsfgsfg\n");
    println!("sdfgewrdfgsdfgsf {} {} {} {}", 235, 56.4653, 567456, 0.435);
    println!("--------------------------------------------");
    */

    init_idt();
    int3();
    loop {}
}
