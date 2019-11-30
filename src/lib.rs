#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;

pub mod gdt;
pub mod interrupts;
pub mod serial;
pub mod vga_buffer;

/// Tests entry point
#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();
    test_main();
    loop {}
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
pub fn init() {
    interrupts::init_idt();
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
    loop {}
}

#[cfg(test)]
#[test_case]
fn test_breakpoint_exception() {
    serial_print!("Testing breakpoint exception\n");
    x86_64::instructions::interrupts::int3();
    serial_print!("[ok]\n");
}
