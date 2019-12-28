//! Mapping of Virtual addresses to Physical Addresses

use x86_64::structures::paging::{OffsetPageTable, PageTable};
use x86_64::{PhysAddr, VirtAddr};

/// Returns a mutable reference to the active level 4 page table
unsafe fn level_four_active_table(physical_mem_offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (level_four_table_frame, _) = Cr3::read();

    let phys = level_four_table_frame.start_address();
    let virt = physical_mem_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    &mut *page_table_ptr
}

/// Translates a virtual address to the mapped physical address

/// NOTE
/// Unused
pub unsafe fn translate_virt_addr(
    addr: VirtAddr,
    physical_mem_offset: VirtAddr,
) -> Option<PhysAddr> {
    translate_addr_inner((addr, physical_mem_offset)) // limit unsafe scope
}

/// Called by `translate_virt_addr` to limit the unsafe scope

/// NOTE: Unused -> The x86 MapperAllSizes provides implementation
/// for translation of huge pages
fn translate_addr_inner((addr, mem_offset): (VirtAddr, VirtAddr)) -> Option<PhysAddr> {
    use x86_64::registers::control::Cr3;
    use x86_64::structures::paging::page_table::FrameError;

    let (level_four_table_frame, _) = Cr3::read();
    let table_indices = [
        addr.p4_index(),
        addr.p3_index(),
        addr.p2_index(),
        addr.p1_index(),
    ];
    let mut frame = level_four_table_frame;

    // traverse Page Table
    for &index in &table_indices {
        // Convert frame to PT reference
        let virt = mem_offset + frame.start_address().as_u64();
        let table_pointer: *const PageTable = virt.as_ptr();
        let table = unsafe { &*table_pointer };

        // Read table entry and update frame
        let entry = &table[index];
        frame = match entry.frame() {
            Ok(frame) => frame,
            Err(FrameError::FrameNotPresent) => return None,
            Err(FrameError::HugeFrame) => panic!("Huge pages not supported"),
        }
    }
    // Pysical Address
    Some(frame.start_address() + u64::from(addr.page_offset()))
}

/// Initializes a new offset page table
///
/// The `OffsetPageTable` assumes the complete physical memory is
/// mapped to the virtual addr space at a certain offset
pub unsafe fn init(physical_mem_offset: VirtAddr) -> OffsetPageTable<'static> {
    // unsafe -> guarantee physical memory mapped to virtual memory at
    // the passed offset
    let level_four_page_table = level_four_active_table(physical_mem_offset);
    OffsetPageTable::new(level_four_page_table, physical_mem_offset)
}