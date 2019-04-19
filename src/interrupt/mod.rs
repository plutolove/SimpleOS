/*
#include <stdint.h>

struct idt_record
{
    uint16_t  limit;      /* Size of IDT array - 1 */
    uintptr_t base;       /* Pointer to IDT array  */
} __attribute__((packed));

void load_idt (struct idt_record *idt_r)
{
    __asm__ ("lidt %0" :: "m"(*idt_r));
}
*/

pub mod table_pointer;
pub mod interrupt_table;

use lazy_static::lazy_static;
use crate::interrupt::interrupt_table::*;
use crate::println;


/// 触发 breakpoint 中断
pub fn int3() {
    unsafe {
        asm!("int3" :::: "volatile");
    }
}

/// breakpoint 中断处理函数
extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT .......\n");
}

lazy_static! {
    static ref IDT: InterruptTable = {
        let mut idt = InterruptTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}