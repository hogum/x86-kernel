//! Heap Allocation tests
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(x86_kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use bootloader::{entry_point, BootInfo};
use core::{panic, PanicInfo};

use alloc::boxed::Box;
use x86_kernel::{serial_print, serial_println};

entry_point!(main);

/// Heap test entry point
fn main(_boot_info: &'static BootInfo) -> ! {
    use x86_kernel::{
        allocator,
        memory::{self, BootInfoFrameAllocator},
    };

    use x86_64::VirtAddr;

    x86_kernel::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };
    allocator::map_heap(&mut mapper, &mut frame_allocator).expect("Failed to initialize heap");
    test_main();
    loop {}
}

#[panic_handler]
/// Test panic handler
fn panic(info: &PanicInfo) -> ! {
    x86_kernel::test_panic_handler(info)
}

/// Tests allocation of value in heap memory
fn test_allocation() -> () {
    serial_println!("Allocation test...");
    let value = Box::new(54);
    assert_eq!(*heap_value, 54);
    serial_println("[ok]");
}
