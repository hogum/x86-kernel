#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(x86_kernel::test_runner)]
#![reexport_test_harness_main = "test_main"] //  Rename generated test entry point from `main`

extern crate alloc;

use core::panic::PanicInfo;

use x86_kernel::{allocator, println};

use alloc::{boxed::Box, rc::Rc, vec, vec::Vec};
use bootloader::{entry_point, BootInfo};

entry_point!(kernel_entry); // Defined the lower level _start()
                            // allowing use of a type-checked Rust function as the entry

/// Linker entry point
pub fn kernel_entry(boot_info: &'static BootInfo) -> ! {
    use x86_64::structures::paging::Page;
    use x86_64::VirtAddr;
    use x86_kernel::memory::{self};

    x86_kernel::init();

    let physical_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(physical_mem_offset) };
    let mut frame_allocator =
        unsafe { memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };

    let heap_item = Box::new("fdf");
    let mut vect = Vec::new();

    for i in 0..1000 {
        vect.push(i);
    }
    println!("vec at {:p}", vect.as_slice());
    println!("Heap value at {:p}", heap_item);

    // let level_four_table = unsafe { level_four_active_table(physical_mem_offset) };

    // map unused page
    let page = Page::containing_address(VirtAddr::new(0));
    memory::create_mapping(page, &mut mapper, &mut frame_allocator);

    // Write something to screen through the new mapping
    // let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    // unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };
    allocator::map_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    let ref_counted_vec = Rc::new(vec![1, 2, 3, 4, 5]);
    let cloned_ref = ref_counted_vec.clone();
    println!("Current ref count - {}", Rc::strong_count(&cloned_ref));
    core::mem::drop(ref_counted_vec);
    println!(
        "Updated reference count - {}",
        Rc::strong_count(&cloned_ref)
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
