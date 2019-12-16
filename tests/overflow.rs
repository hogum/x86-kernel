#![feature(abi_x86_interrupt)]
//! Double fault stack tests
//!

#![no_std]
#![no_main]

use core::panic::PanicInfo;
use lazy_static::lazy_static;

use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

use x86_kernel::{exit_qemu, serial_println, QemuExitCode};

lazy_static! {
    /// Test Interrupt Descriptor Table
    /// Stack Overflow Test needs its own IDT with a custom Double Fault handler
    static ref TEST_IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        unsafe {
            idt.double_fault
                .set_handler_fn(test_double_fault_handler)
                .set_stack_index(x86_kernel::gdt::DOUBLE_FAULT_IST_IDX);
        }
        idt
    };
}

/// Initializes the test IDT on the CPU
pub fn init_test_idt() -> () {
    TEST_IDT.load();
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    serial_println!("Starting Overflow test...");

    x86_kernel::gdt::init();
    init_test_idt();

    overflow_stack();

    panic!("Execution occurred after stack overflow");
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    x86_kernel::test_panic_handler(info)
}

#[allow(unconditional_recursion)]
fn overflow_stack() {
    overflow_stack(); // Push return address on each recursion
}

/// Marks test as passed by exiting Qemu with a
/// Success exit
extern "x86-interrupt" fn test_double_fault_handler(
    _stack_frame: &mut InterruptStackFrame,
    _error_code: u64,
) {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}
