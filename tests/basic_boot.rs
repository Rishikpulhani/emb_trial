#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)] // here test_runner was the func name which will be run when we do cargo test
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // the tests of this crate will use this as the paninc handler not the one in main
    loop {}
}
fn test_runner(tests: &[&dyn Fn()]){
    unimplemented!();
}
#[no_mangle]
pub extern "C" fn _start() -> !{
    test_main();
    loop {}
}

