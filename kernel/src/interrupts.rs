use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::println;

lazy_static::lazy_static!{
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.double_fault.set_handler_fn(double_fault_handler);
        idt
    };
}
pub fn init_idt(){
    IDT.load();
}
extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame){
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}
extern "x86-interrupt" fn double_fault_handler(stack_frame: InterruptStackFrame, error_code: u64) -> !{ // diverging as the x86_64 architecture does not permit returning from a double fault exception.
    println!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
    println!("EXCEPTION: DOUBLE FAULT\n{:#?}", error_code); // 0 by default 
    loop {}
}