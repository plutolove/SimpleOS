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
    init_idt();
    int3();
    loop {}
}
