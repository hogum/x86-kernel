#![no_std]
#![feature(alloc_error_handler)]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;

extern crate alloc;

pub mod allocator;
pub mod gdt;
pub mod interrupts;
pub mod memory;
pub mod serial;
pub mod vga_buffer;

/// Global Allocator
/// Allocator instance to be used as the global heap allocator
#[global_allocator]
static ALLOCATOR: allocator::SimpleAlloc = allocator::SimpleAlloc;

/// Called on allocation failure
#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("Heap alloc error: {:?}", layout)
}

/// Tests entry point
#[cfg(test)]
use bootloader::{entry_point, BootInfo};

#[cfg(test)]
entry_point!(test_kernel_entry);

/// Entry point for `cargo xtest`
#[cfg(test)]
pub fn test_kernel_entry(_boot_info: &'static BootInfo) -> ! {
    init();
    test_main();
    halt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum QemuExitCode {
    Success = 0x10,
    Failure = 0x11,
}

pub fn exit_qemu(code: QemuExitCode) -> () {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(code as u32);
    }
}

/// General Initializer for the exceptions
/// Initializes by calling `init_idt`
/// Loads the GDT
pub fn init() {
    gdt::init();
    interrupts::init_idt();
    unsafe {
        interrupts::PICS.lock().initialize();
    }
    x86_64::instructions::interrupts::enable();
}

/// Halts the CPU until the next interrupt arrives
/// The CPU is made to go to sleep while idle
pub fn halt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

pub fn test_runner(tests: &[&dyn Fn()]) -> () {
    serial_println!("Running {} tests", tests.len());

    for test in tests {
        test();
    }
    exit_qemu(QemuExitCode::Success);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[oops!]");
    serial_println!("\nError: {}\n", info);
    exit_qemu(QemuExitCode::Failure);
    halt_loop();
}

#[cfg(test)]
#[test_case]
fn test_breakpoint_exception() {
    serial_print!("Testing breakpoint exception\n");
    x86_64::instructions::interrupts::int3();
    serial_print!("[ok]\n");
}
