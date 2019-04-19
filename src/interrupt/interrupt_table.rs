use core::marker::PhantomData;
use bit_field::BitField;
use bitflags::bitflags;

bitflags! {
    /// Describes an page fault error code.
    #[repr(transparent)]
    pub struct PageFaultErrorCode: u64 {
        /// If this flag is set, the page fault was caused by a page-protection violation,
        /// else the page fault was caused by a not-present page.
        const PROTECTION_VIOLATION = 1 << 0;

        /// If this flag is set, the memory access that caused the page fault was a write.
        /// Else the access that caused the page fault is a memory read. This bit does not
        /// necessarily indicate the cause of the page fault was a read or write violation.
        const CAUSED_BY_WRITE = 1 << 1;

        /// If this flag is set, an access in user mode (CPL=3) caused the page fault. Else
        /// an access in supervisor mode (CPL=0, 1, or 2) caused the page fault. This bit
        /// does not necessarily indicate the cause of the page fault was a privilege violation.
        const USER_MODE = 1 << 2;

        /// If this flag is set, the page fault is a result of the processor reading a 1 from
        /// a reserved field within a page-translation-table entry.
        const MALFORMED_TABLE = 1 << 3;

        /// If this flag is set, it indicates that the access that caused the page fault was an
        /// instruction fetch.
        const INSTRUCTION_FETCH = 1 << 4;
    }
}

//TODO: implement interrupt stack frame
pub struct InterruptStackFrame(u64);

/// A handler function for an interrupt or an exception without error code.
pub type HandlerFunc = extern "x86-interrupt" fn(&mut InterruptStackFrame);

/// A handler function for an exception that pushes an error code.
pub type HandlerFuncWithErrCode = extern "x86-interrupt" fn(&mut InterruptStackFrame, error_code: u64);

/// A page fault handler function that pushes a page fault error code.
pub type PageFaultHandlerFunc = extern "x86-interrupt" fn(&mut InterruptStackFrame, error_code: PageFaultErrorCode);


pub fn csr() -> u16 {
    let mut segment: u16 = 0;
    unsafe { asm!("mov %cs, $0" : "=r" (segment) ) };
    segment
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EntryOption(u16);

impl EntryOption {
    const fn new() -> Self {
        EntryOption(0b1110_0000_0000)
    }

    pub fn set_present(&mut self, present: bool) -> &mut Self {
        self.0.set_bit(15, present);
        self
    }

    pub fn disable_interrupts(&mut self, disable: bool) -> &mut Self {
        self.0.set_bit(8, !disable);
        self
    }

    pub unsafe fn set_stack_index(&mut self, index: u16) -> &mut Self {
        self.0.set_bits(0..3, index + 1);
        self
    }
}


#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Entry<F> {
    p_low: u16,
    gdt_selector: u16,
    option: EntryOption,
    p_mid: u16,
    p_high: u32,
    reserved: u32,
    phantom: PhantomData<F>,
}


impl<F> Entry<F> {
    pub const fn missing() -> Self {
        Entry {
            gdt_selector: 0,
            p_low: 0,
            p_mid: 0,
            p_high: 0,
            option: EntryOption::new(),
            reserved: 0,
            phantom: PhantomData,
        }
    }

    #[cfg(target_arch = "x86_64")]
    fn set_handler_addr(&mut self, addr: u64) -> &mut EntryOption {
        self.p_low = addr as u16;
        self.p_mid = (addr >> 16) as u16;
        self.p_high = (addr >> 32) as u32;
        self.gdt_selector = csr();
        self.option.set_present(true);
        &mut self.option
    }
}

macro_rules! impl_set_handler_fn {
    ($h:ty) => {
        #[cfg(target_arch = "x86_64")]
        impl Entry<$h> {
            pub fn set_handler_fn(&mut self, handler: $h) -> &mut EntryOption {
                self.set_handler_addr(handler as u64)
            }
        }
    };
}

impl_set_handler_fn!(HandlerFunc);
impl_set_handler_fn!(HandlerFuncWithErrCode);
impl_set_handler_fn!(PageFaultHandlerFunc);

#[allow(missing_debug_implementations)]
#[derive(Clone)]
#[repr(C)]
#[repr(align(16))]
pub struct InterruptTable {
    pub divide_by_zero: Entry<HandlerFunc>,

    pub debug: Entry<HandlerFunc>,

    pub non_maskable_interrupt: Entry<HandlerFunc>,

    pub breakpoint: Entry<HandlerFunc>,

    pub overflow: Entry<HandlerFunc>,


    pub bound_range_exceeded: Entry<HandlerFunc>,

    pub invalid_opcode: Entry<HandlerFunc>,

    pub device_not_available: Entry<HandlerFunc>,

    pub double_fault: Entry<HandlerFuncWithErrCode>,

    coprocessor_segment_overrun: Entry<HandlerFunc>,

    pub invalid_tss: Entry<HandlerFuncWithErrCode>,

    pub segment_not_present: Entry<HandlerFuncWithErrCode>,

    pub stack_segment_fault: Entry<HandlerFuncWithErrCode>,

    pub general_protection_fault: Entry<HandlerFuncWithErrCode>,

    pub page_fault: Entry<PageFaultHandlerFunc>,

    reserved_1: Entry<HandlerFunc>,

    pub x87_floating_point: Entry<HandlerFunc>,

    pub alignment_check: Entry<HandlerFuncWithErrCode>,

    pub machine_check: Entry<HandlerFunc>,

    pub simd_floating_point: Entry<HandlerFunc>,

    pub virtualization: Entry<HandlerFunc>,

    reserved_2: [Entry<HandlerFunc>; 9],

    pub security_exception: Entry<HandlerFuncWithErrCode>,

    reserved_3: Entry<HandlerFunc>,

    interrupts: [Entry<HandlerFunc>; 256 - 32],
}


impl InterruptTable {
    pub const fn new() -> InterruptTable {
        InterruptTable {
            divide_by_zero: Entry::missing(),
            debug: Entry::missing(),
            non_maskable_interrupt: Entry::missing(),
            breakpoint: Entry::missing(),
            overflow: Entry::missing(),
            bound_range_exceeded: Entry::missing(),
            invalid_opcode: Entry::missing(),
            device_not_available: Entry::missing(),
            double_fault: Entry::missing(),
            coprocessor_segment_overrun: Entry::missing(),
            invalid_tss: Entry::missing(),
            segment_not_present: Entry::missing(),
            stack_segment_fault: Entry::missing(),
            general_protection_fault: Entry::missing(),
            page_fault: Entry::missing(),
            reserved_1: Entry::missing(),
            x87_floating_point: Entry::missing(),
            alignment_check: Entry::missing(),
            machine_check: Entry::missing(),
            simd_floating_point: Entry::missing(),
            virtualization: Entry::missing(),
            reserved_2: [Entry::missing(); 9],
            security_exception: Entry::missing(),
            reserved_3: Entry::missing(),
            interrupts: [Entry::missing(); 256 - 32],
        }
    }

    pub fn reset(&mut self) {
        self.divide_by_zero = Entry::missing();
        self.debug = Entry::missing();
        self.non_maskable_interrupt = Entry::missing();
        self.breakpoint = Entry::missing();
        self.overflow = Entry::missing();
        self.bound_range_exceeded = Entry::missing();
        self.invalid_opcode = Entry::missing();
        self.device_not_available = Entry::missing();
        self.double_fault = Entry::missing();
        self.coprocessor_segment_overrun = Entry::missing();
        self.invalid_tss = Entry::missing();
        self.segment_not_present = Entry::missing();
        self.stack_segment_fault = Entry::missing();
        self.general_protection_fault = Entry::missing();
        self.page_fault = Entry::missing();
        self.reserved_1 = Entry::missing();
        self.x87_floating_point = Entry::missing();
        self.alignment_check = Entry::missing();
        self.machine_check = Entry::missing();
        self.simd_floating_point = Entry::missing();
        self.virtualization = Entry::missing();
        self.reserved_2 = [Entry::missing(); 9];
        self.security_exception = Entry::missing();
        self.reserved_3 = Entry::missing();
        self.interrupts = [Entry::missing(); 256 - 32];
    }

    #[cfg(target_arch = "x86_64")]
    pub fn load(&'static self) {
        use crate::interrupt::table_pointer::{lidt, TablePointer};
        use core::mem::size_of;
        let ptr = TablePointer {
            base: self as *const _ as u64,
            limit: (size_of::<Self>() - 1) as u16,
        };

        unsafe {
            lidt(&ptr);
        }
    }
}