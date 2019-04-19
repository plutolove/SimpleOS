#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct TablePointer {
    /// Size of the DT.
    pub limit: u16,
    /// Pointer to the memory region containing the DT.
    pub base: u64,
}

pub unsafe fn lidt(idt: &TablePointer) {
    asm!("lidt ($0)" :: "r" (idt) : "memory");
}
