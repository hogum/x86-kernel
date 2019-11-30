//! Handles implementations for the Global Descriptor Table

use lazy_static::lazy_static;
use x86_64::{structures::tss::TaskStateSegment, VirtAddr};

pub const DOUBLE_FAULT_IST_IDX: u16 = 0;

lazy_static! {
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();

        // Write address of Double fault stack to entry 0
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_IDX as usize] = {
            const STACK_SIZE: usize = 4096;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            let mut stack_entry = VirtAddr::from_ptr(unsafe { &STACK });// Accessing mut static
            let stack_end = stack_entry + STACK_SIZE;

            // Stacks grow downwards, so write the top address
            stack_end
        };
        tss
    };
}
