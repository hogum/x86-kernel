#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
//  Rename generated test entry point from `main`
#![reexport_test_harness_main = "test_main"]

mod serial;
mod vga_buffer;

use core::panic::PanicInfo;

/// Panic Handler
///
/// Called by the compiler on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
/// Linker entry point
pub extern "C" fn _start() -> ! {
    println!("Some sodadust {}", "on buckets");

    #[cfg(test)]
    test_main();

    loop {}
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn oop_trial() {
    println!("Attempting something");
    assert_ne!(2, 4);
    println!("[ok]");
}

#[derive(Debug, Clone, Eq, PartialEq, Copy)]
#[repr(u32)]
/// Represents the status to Qemu
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}
