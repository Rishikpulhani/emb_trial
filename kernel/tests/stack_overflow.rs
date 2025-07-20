#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
// #![test_runner(kernel::test_runner)] // here test_runner was the func name which will be run when we do cargo test
// #![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use kernel::{println,init,framebuffer::FrameBufferWriter,QemuExitCode,exit_qemu,serial_println};
use bootloader_api::{entry_point,BootInfo};
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    kernel::test_panic_handler(_info);
}

entry_point!(test_kernel_main);

fn test_kernel_main(boot_info: &'static mut BootInfo) -> ! {
    let framebuffer =  boot_info.framebuffer.as_mut().expect("Framebuffer not available");
    let framebuffer_info = framebuffer.info();
    FrameBufferWriter::new(framebuffer.buffer_mut(), framebuffer_info);
    serial_println!("stack_overflow::stack_overflow...\t");
    kernel::gdt::init_gdt();
    init_test_idt();
    stack_overflow();
    panic!("Execution continued after stack overflow");
}
#[allow(unconditional_recursion)]
fn stack_overflow() {
    stack_overflow(); // for each recursion, the return address is pushed
    volatile::Volatile::new(0).read(); // prevent tail recursion optimizations
}

lazy_static::lazy_static!{
    static ref TEST_IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        unsafe {
            idt.double_fault.set_handler_fn(test_double_fault_handler).set_stack_index(0);
            // unsafe as need to ensure that the stack assigned is valid
        }
        idt
    };
}
pub fn init_test_idt(){
    TEST_IDT.load();
}
extern "x86-interrupt" fn test_double_fault_handler(stack_frame: InterruptStackFrame, error_code: u64) -> !{ 
    // diverging as the x86_64 architecture does not permit returning from a double fault exception.
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}