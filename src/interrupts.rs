/// Interrupts
use x86_64::structures::idt::InterruptDescriptorTable;

/// Creates the IDT
pub fn init_idt() -> () {
    let mut idt = InterruptDescriptorTable::new();
}
