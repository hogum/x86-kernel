#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(x86_kernel::test_runner)]
#![reexport_test_harness_main = "test_main"] //  Rename generated test entry point from `main`

use core::panic::PanicInfo;

use x86_kernel::println;

/// Linker entry point
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Some sodadust {}", "on buckets");

    x86_kernel::init();
    // INT3 invokes a breapoint exception
    x86_64::instructions::interrupts::int3();

    fn overflow_stack() {
        // Push the return address for each recursion
        overflow_stack();
    }
    overflow_stack();
    unsafe {
        *(0xdeadbeef as *mut u64) = 42;
    }

    #[cfg(test)]
    test_main();

    println!("Completed without crash");
    loop {
        use x86_kernel::println;
        // Introduce deadlock
        // Interrupt handler should try to print while
        // the print WRITER is locked
        println!("!");
    }
}

/// Panic Handler
///
/// Called by the compiler on panic
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    x86_kernel::test_panic_handler(info)
}
