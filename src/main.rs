#![no_std]
#![no_main]

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
    loop {}
}
