#![no_std]
#![feature(abi_x86_interrupt)]
#![cfg_attr(test, no_main)] // Since our lib.rs is tested independently of our main.rs, we need to add a _start entry point and a panic handler when the library is compiled in test mode. By using the cfg_attr crate attribute, we conditionally enable the no_main attribute in this case.
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)] // these tests are for the unit tests for this lib
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use crate::{framebuffer::FrameBufferWriter, gdt::init_gdt};
use crate::interrupts::{init_idt,PICS};

pub mod serial;
pub mod framebuffer;
pub mod interrupts;
pub mod gdt;
//pub mod vga_buffer;
//support for running tests
pub trait Testable {
    fn run(&self);
}
impl <T> Testable for T
where T: Fn() {
    // all items implementing Fn will also implement testable 
    fn run(&self) {
        serial_print!("{}...\t",core::any::type_name::<T>());
        self();
        serial_println!("[ok]"); // if ok is prinited then it means that the test didnt panic
    }
}

//not add this here as this tells to only compile this portion when running unit tests in the same crate but integration tests are built ion a different crate and so we need to make this accessible to them 
//#[cfg(test)]
pub fn test_runner(tests: &[&dyn Testable]) {
    //tests - It is basically a list of references to types that can be called like a function.
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}
#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}

// for both unit and integration tests and also other binaries using it 
pub fn test_panic_handler(_info: &PanicInfo) -> ! {
    serial_println!("[failed]\n"); // as panic only happens when a test fails 
    serial_println!("Error: {}\n", _info);
    exit_qemu(QemuExitCode::Failed); // since we are seeing the error in the console we dont need to the qemu to be running so close it 
    hlt_loop();
}
#[cfg(test)] // only for unit tests
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info) // we seperated this out so that we can make the same handler available to executables as well just like we do in case of std lib
}



// #[cfg(test)] // only for unit tests
// #[unsafe(no_mangle)]
// pub extern "C" fn _start() -> ! {
//     init();
//     test_main();
//     loop{}
// }
// this _start is for the older version of the bootloader crate 
// _ start for newer version of bootloader crate


#[cfg(test)]
use bootloader_api::{entry_point, BootInfo};

#[cfg(test)]
entry_point!(test_kernel_main);

#[cfg(test)]
fn test_kernel_main(boot_info: &'static mut BootInfo) -> ! {
    let framebuffer =  boot_info.framebuffer.as_mut().expect("Framebuffer not available");
    let framebuffer_info = framebuffer.info();
    FrameBufferWriter::new(framebuffer.buffer_mut(), framebuffer_info);
    init();
    test_main();
    hlt_loop();
}

//support for qemu ops
#[repr(u32)]
pub enum QemuExitCode {
    // this is a way to stop the execution of this os we are building and since it never happens in real env we only use this in testing env, when the qemu stops which is teh one executing our os binary virtually then it will stop the execution of this binary as well 
    // each one is 8 bit as in hex
    Success = 0x10, //16 + 0
    Failed = 0x11,  //16 + 1
}
pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;
    let mut port = Port::new(0xf4); // creating a port is safe but accessing it is not
    unsafe {
        port.write(exit_code as u32);
    } // exit_code should implement the portwrite trait which is done only by u8,u16,u32
}

// INTERRUPTS 
pub fn init(){
    init_gdt();
    init_idt();
    unsafe {
        PICS.lock().initialize();
    }
    x86_64::instructions::interrupts::enable();
}
pub fn hlt_loop() -> !{
    loop {
        x86_64::instructions::hlt();
    }
}