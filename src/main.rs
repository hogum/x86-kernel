mod vga_buffer;
#![no_std]
#![no_main]


use core::panic::PanicInfo;

/// Panic Handler
///
/// Called by the compiler on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
static VISUAL: &[u8] = b"Underppined legs";
#[no_mangle]
/// Linker entry point
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in VISUAL.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    loop {}
}
