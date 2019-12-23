#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(x86_kernel::test_runner)]
#![reexport_test_harness_main = "test_main"] //  Rename generated test entry point from `main`

use core::panic::PanicInfo;

use x86_kernel::println;

use bootloader::{entry_point, BootInfo};

entry_point!(kernel_entry); // Defined the lower level _start()
                            // allowing use of a type-checked Rust function as the entry

/// Linker entry point
pub fn kernel_entry(boot_info: &'static BootInfo) -> ! {
    println!("Some sodadust {}", "on buckets");

    x86_kernel::init();

    use x86_64::registers::control::Cr3;

    let (level_four_page_table, _) = Cr3::read(); // (PhysFrame, Cr3Flags)
    println!(
        "Level four page table at: {:#?}",
        level_four_page_table.start_address()
    );
    #[cfg(test)]
    test_main();

    println!("Completed without crash");
    x86_kernel::halt_loop();
}

/// Panic Handler
///
/// Called by the compiler on panic
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    x86_kernel::halt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    x86_kernel::test_panic_handler(info)
}
