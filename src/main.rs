#![no_std]
#![no_main]

mod vga_buffer;

use core::fmt::Write;
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
    vga_buffer::WRITER
        .lock()
        .write_str("Hehe! About to get...")
        .unwrap();
    write!(
        vga_buffer::WRITER.lock(),
        " , some odd stuff: {} {}",
        32.9,
        4
    )
    .unwrap();
    loop {}
}
