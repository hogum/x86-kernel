/// Interrupts
///
use crate::{gdt, println};
use lazy_static::lazy_static;

use pic8259_simple::ChainedPics;
use spin::Mutex;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: Mutex<ChainedPics> =
    Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

/// Specifies the Index for each interrupt variant
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    /// Timer interupt - Line 0 of PIC
    Timer = PIC_1_OFFSET,

    /// Keyboard uses line 1 of PIC
    /// interrupt (1 + offset 32)
    Keyboard, // Defaults previous value + 1
}

impl InterruptIndex {
    fn as_u8(self) -> u8 {
        self as u8
    }

    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}

lazy_static! {
    /// Interrupt DT
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
        idt.double_fault.set_handler_fn(double_fault_handler).set_stack_index(gdt::DOUBLE_FAULT_IST_IDX);
        }
        idt[InterruptIndex::Timer.as_usize()].set_handler_fn(timer_er_interrupt_handler);
        idt[InterruptIndex::Keyboard.as_usize()].set_handler_fn(keyboard_interrupt_handler);
        idt
    };
}

use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

/// Creates the IDT
pub fn init_idt() -> () {
    IDT.load();
}

/// Handles breakpoint Exceptions
extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut InterruptStackFrame) {
    println!("Oops! Exception:\n\t{:#?}", stack_frame);
}

/// Handles Double Faults
extern "x86-interrupt" fn double_fault_handler(
    stack_frame: &mut InterruptStackFrame,
    _error_code: u64,
) {
    panic!("Exception: Double Fault\n{:#?}", stack_frame);
}

/// Handler function for the timer interrupt
/// Implements the CPU reaction to the timer exception
extern "x86-interrupt" fn timer_er_interrupt_handler(_stack_frame: &mut InterruptStackFrame) {
    println!("Timer interrupt");
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}

/// Handles Keyboard interrupts
extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: &mut InterruptStackFrame) {
    println!("keyboard interrupt");
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}
