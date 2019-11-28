/// Interrupts
///
use crate::println;
use lazy_static::lazy_static;

lazy_static! {
    /// Interrupt DT
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.double_fault.set_handler_fn(double_fault_handler);
        idt
    };
}

use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

/// Creates the IDT
pub fn init_idt() -> () {
    IDT.load();
}

/// Handles breakpoint Exceptions
extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut InterruptStackFrame) {
    println!("Oops! Exception:\n\t{:#?}", stack_frame);
}

/// Handles Double Faults
extern "x86-interrupt" fn double_fault_handler(
    stack_frame: &mut InterruptStackFrame,
    _error_code: u64,
) {
    panic!("Exception: Double Fault\n{:#?}", stack_frame);
}
