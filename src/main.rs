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
    use x86_64::structures::paging::MapperAllSizes;
    use x86_64::VirtAddr;
    use x86_kernel::memory::{self};

    println!("Some sodadust {}", "on buckets");

    x86_kernel::init();

    let physical_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mapper = unsafe { memory::init(physical_mem_offset) };
    // let level_four_table = unsafe { level_four_active_table(physical_mem_offset) };

    let addresses = [
        0xb8000,                          // identity-mapped VGA  buffer page
        0x201008,                         // A code page
        0x0100_0020_1a10,                 // A stack page
        boot_info.physical_memory_offset, // virtual addr mapped to physical addr 0
    ];

    println!("Virtual -> Physical");
    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = mapper.translate_addr(virt);
        //  let phys = unsafe { translate_virt_addr(virt, physical_mem_offset) };
        println!("{:?} -> {:?}", virt, phys);
    }

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
