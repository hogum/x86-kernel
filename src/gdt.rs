//! Handles implementations for the Global Descriptor Table

use lazy_static::lazy_static;
use x86_64::structures::gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector};
use x86_64::structures::tss::TaskStateSegment;
use x86_64::VirtAddr;

pub const DOUBLE_FAULT_IST_IDX: u16 = 0;

lazy_static! {
    /// Global DescriptorTable instance
    static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));
        (gdt, Selectors {code_selector, tss_selector})
    };
}

/// Segment selectors
/// Loads the cs and tss
struct Selectors {
    code_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}

lazy_static! {
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();

        // Write address of Double fault stack to entry 0
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_IDX as usize] = {
            const STACK_SIZE: usize = 4096;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            let stack_entry = VirtAddr::from_ptr(unsafe { &STACK });// Accessing mut static
            let stack_end = stack_entry + STACK_SIZE;

            // Stacks grow downwards, so write the top address
            stack_end
        };
        tss
    };
}

/// Loads the GDT
pub fn init() {
    use x86_64::instructions::segmentation::set_cs;
    use x86_64::instructions::tables::load_tss;

    GDT.0.load();

    unsafe {
        set_cs(GDT.1.code_selector);
        load_tss(GDT.1.tss_selector);
    }
}
