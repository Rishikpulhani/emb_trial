use x86_64::{instructions::port, structures::{gdt, idt::{InterruptDescriptorTable, InterruptStackFrame}}};
use crate::{println,exit_qemu,print};
use pic8259::ChainedPics;
use lazy_static::lazy_static;
use spin::Mutex;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: Mutex<ChainedPics> = Mutex::new(unsafe { //mutex initialisation new function is marked const so can run at compile time , same with th4e chained pic new function 
    ChainedPics::new(PIC_1_OFFSET,PIC_2_OFFSET) // unsafe because wrong offsets could cause undefined behavior.
});

lazy_static!{
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler).set_stack_index(crate::gdt::DOUBLE_FAULT_IST_INDEX);
            // unsafe as need to ensure that the stack assigned is valid
        }
        idt[InterruptIndex::TIMER as u8].set_handler_fn(timer_interrupt_handler);
        idt[InterruptIndex::KeyBoard as u8].set_handler_fn(keyboard_interrupt_handler);
        idt
    };
}
pub fn init_idt(){
    IDT.load();
}    

#[repr(u8)]
pub enum InterruptIndex {
    TIMER = PIC_1_OFFSET,
    KeyBoard, // by deafualt taken as previous value + 1
}
// no need of this code in the updated version of the library as new method directly uses u8
// impl InterruptIndex {
//     fn as_u8(self) -> u8{
//         self as u8
//     }
//     fn as_usize(self) -> usize{
//         usize::from(self.as_u8())
//     }
// }

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame){
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}
extern "x86-interrupt" fn double_fault_handler(stack_frame: InterruptStackFrame, error_code: u64) -> !{ 
    // diverging as the x86_64 architecture does not permit returning from a double fault exception.
    println!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
    println!("EXCEPTION: DOUBLE FAULT\n{:#?}", error_code); // 0 by default 
    loop {}
}
extern "x86-interrupt" fn timer_interrupt_handler(stack_frame: InterruptStackFrame){
    print!(".");
    unsafe {
        PICS.lock().notify_end_of_interrupt(InterruptIndex::TIMER as u8);
    }
}
extern "x86-interrupt" fn keyboard_interrupt_handler(stack_frame: InterruptStackFrame){
    let mut port = port::Port::new(0x60);
    let scancode: u8 = unsafe {
        port.read()
    };
    let key = match scancode {
        0x02 => Some('1'),
        0x03 => Some('2'),
        0x04 => Some('3'),
        0x05 => Some('4'),
        0x06 => Some('5'),
        0x07 => Some('6'),
        0x08 => Some('7'),
        0x09 => Some('8'),
        0x0a => Some('9'),
        0x0b => Some('0'),
        _ => None,
    };
    if let Some(key) = key{
        print!("{}",key);
    }
    // print!("{scancode}"); // internally the print function has the logic to stop interrupts during execution
    unsafe {
        PICS.lock().notify_end_of_interrupt(InterruptIndex::KeyBoard as u8);
    }
}
#[cfg(test)]
#[test_case]
fn test_breakpoint_exception() {
    x86_64::instructions::interrupts::int3();
}