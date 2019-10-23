#![no_std]
use core::panic::PanicInfo;

/// Panic Handler
///
/// Called by the compiler on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

fn main() {}
