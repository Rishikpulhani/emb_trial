#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
//#![test_runner(test_runner)] // here test_runner will be of this crate as we need a different functionality
//#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use emb_trial::{exit_qemu, serial_println,QemuExitCode};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success); // stops execution of this crate altogether - so can run only 1 test in this crate 
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> !{
    //test_main();
    // in harness false no test main is generated 
    should_fail();
    serial_println!("[test did not panic]");
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

// Since the runner always exits after running a single test, it does not make sense to define more than one #[test_case] function.
//#[test_case]
fn should_fail(){
    serial_println!("should_panic::should_fail...\t");
    assert_eq!(0,1);
}