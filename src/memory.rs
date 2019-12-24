//! Mapping of Virtual addresses to Physical Addresses

use x86_64::{structures::paging::PageTable, VirtualAddr};

/// Returns a mutable reference to the active level 4 page table
pub unsafe fn level_four_active_table(pysical_mem_offset: VirtualAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (level_four_table_frame, _) = Cr3::read();

    let phys = level_four_table_frame.start_address();
    let virt = physical_mem_offset + phy.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    &mut *page_table_ptr
}
