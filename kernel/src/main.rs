#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kernel::test_runner)] // here test_runner was the func name which will be run when we do cargo test - since it is now moved to the lib crate se e call it from there - there it is not made #[cfg(test)] as then it will only be accessible to the lib binaray in test mode for unit testing of the lib, and not to other crates or binaraies or the integration executables 
//when we call this test runner in this binary it will collect all tests in this particular binary and run them      
#![reexport_test_harness_main = "test_main"]
// these are still here as we still can test code here as need arises

use bootloader_api::{entry_point, BootInfo};
use core::{fmt::Write, panic::PanicInfo};
use kernel::{hlt_loop, println};
use kernel::framebuffer::FrameBufferWriter;
use kernel::init;
use x86_64::registers::control::Cr3;

//use crate::vga_buffer;
 entry_point!(kernel_main); // The provided entry_point macro will encode the configuration settings into a separate ELF section of the compiled kernel executable., the kernel main is the frst argument and the second is optional and is to be used if we want some other config for our kernel elf. The macro checks the signature of your entry point function and generates a _start entry point symbol for it.

#[cfg(not(test))]
#[panic_handler] // no need to mark the panic function with no_mangle as it is not refered by its name while linking instead it is marked as the panic handler to identify it uniquely
fn panic(_info: &PanicInfo) -> ! {
    use kernel::hlt_loop;

    println!("{}", _info);
    hlt_loop();
}
#[cfg(test)] // only for unit tests
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kernel::test_panic_handler(info);
}

fn kernel_main(boot_info: &'static mut BootInfo) -> ! { // noneed of extern c as the entry point macro itself exposes this func as a _start symbol for lnking 
    // here this is a rust function which is accessed from outside but this happens at the c abi
    let framebuffer =  boot_info.framebuffer.as_mut().expect("Framebuffer not available");
    let framebuffer_info = framebuffer.info();
    FrameBufferWriter::new(framebuffer.buffer_mut(), framebuffer_info);
    //panic!("Some panic message");
    init(); // we create such init functions because these are lazy statics and are initialised at runtime when they are called
    //x86_64::instructions::interrupts::int3();
    // let ptr = 0xdeadbeaf as *mut u8;
    // unsafe { *ptr = 42; }
    let (level_4_page_table, _) = Cr3::read();
    println!("Level 4 page table at: {:?}", level_4_page_table.start_address());
    let ptr = 0x10e3000 as *mut u8;

    // read from a code page
    unsafe { let x = *ptr; }
    println!("read worked");

    // write to a code page
    //unsafe { *ptr = 42; }
    println!("write worked");
    println!("It did not crash!");
    for i in 1..1000 {
        println!("It did not crash!");
    } // deadlock provocation - non deterministic to determine the number of print which will be executed as depends on when asyncly the timer interrupt occurs 
    #[cfg(test)] // for unit tests related to this crate only 
    test_main();
    // loop {}
    hlt_loop();
}




