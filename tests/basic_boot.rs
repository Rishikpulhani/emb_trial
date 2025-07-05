#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(emb_trial::test_runner)] // here test_runner was the func name which will be run when we do cargo test
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use emb_trial::println;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    emb_trial::test_panic_handler(_info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> !{
    test_main();
    loop {}
}

#[test_case]
fn test_println() {
    // repeated test from vga buffer as both will be tested in different environments later
    // vga buffer is in the lib crate and works even before initialisation routine of the os in the _start function - so this one tests it without any initialisation routine, in lib it will be tested after the initialisation routines of the os
    println!("test_println output");
}