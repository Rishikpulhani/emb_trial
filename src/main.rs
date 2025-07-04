#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(emb_trial::test_runner)] // here test_runner was the func name which will be run when we do cargo test - since it is now moved to the lib crate se e call it from there - there it is not made #[cfg(test)] as then it will only be accessible to the lib binaray in test mode for unit testing of the lib, and not to other crates or binaraies or the integration executables 
//when we call this test runner in this binary it will collect all tests in this particular binary and run them      
#![reexport_test_harness_main = "test_main"]
// these are still here as we still can test code here as need arises

use core::panic::PanicInfo;
use emb_trial::println;

//use crate::vga_buffer;

#[cfg(not(test))]
#[panic_handler] // no need to mark the panic function with no_mangle as it is not refered by its name while linking instead it is marked as the panic handler to identify it uniquely
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}
#[cfg(test)] // only for unit tests
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    emb_trial::test_panic_handler(info) 
}

//static HELLO: &[u8] = b"Hello World!";
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    // here this is a rust function which is accessed from outside but this happens at the c abi
    // let vga_buffer = 0xb8000 as *mut u8;

    // for (i, &byte) in HELLO.iter().enumerate() {
    //     unsafe {
    //         *vga_buffer.offset(i as isize * 2) = byte;
    //         *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
    //     }
    // }
    //vga_buffer::print_something();
    // use core::fmt::Write;
    // vga_buffer::WRITER.lock().write_byte(b'H');
    // vga_buffer::WRITER.lock().write_string("ello ");
    // write!(vga_buffer::WRITER.lock(), "The numbers are {} and {}", 42, 1.0 / 3.0).unwrap();
    // no import as already in the root namespace as macro export
    //panic!("Some panic message");
    // for i in 1..100{
    //     println!("{i}");
    // }
    println!("Hello World{}", "!");
    #[cfg(test)]
    test_main();
    loop {}
}




