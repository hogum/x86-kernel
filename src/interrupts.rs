/// Interrupts
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

/// Creates the IDT
pub fn init_idt() -> () {
    let mut idt = InterruptDescriptorTable::new();
    idt.breakpoint.set_handler_fn(breakpoint_handler);
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut InterruptStackFrame) {
    println!("Oops! Exception:\n\t{:#?}", stack_frame);
}
