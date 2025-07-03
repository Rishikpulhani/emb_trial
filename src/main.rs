#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)] // here test_runner was the func name which will be run when we do cargo test
#![reexport_test_harness_main = "test_main"]
use core::panic::PanicInfo;

//use crate::vga_buffer;
mod serial;
mod vga_buffer;
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}
#[cfg(test)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[failed]\n"); // as panic only happens when a test fails 
    serial_println!("Error: {}\n", _info);
    exit_qemu(QemuExitCode::Failed); // since we are seeing the error in the console we dont need to the qemu to be running so close it 
    loop {}
}
// no need to mark the panic function with no_mangle as it is not refered by its name while linking instead it is marked as the panic handler to identify it unique;y
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
    println!("Hello World{}", "!"); // no import as already in the root namespace as macro export
                                    //panic!("Some panic message");
                                    // for i in 1..100{
                                    //     println!("{i}");
                                    // }
    #[cfg(test)]
    test_main();
    loop {}
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    //tests - It is basically a list of references to types that can be called like a function.
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();

    }
    exit_qemu(QemuExitCode::Success);
}
#[test_case]
fn trivial_assertion() {
    serial_print!("trivial assertion... ");
    assert_eq!(1, 1);
    //assert_eq!(1, 2); // on fqailing this calls the panic handler and so qemu never exits 
    serial_println!("[ok]");
}

#[repr(u32)]
enum QemuExitCode {
    // each one is 8 bit as in hex
    Success = 0x10, //16 + 0
    Failed = 0x11,  //16 + 1
}
fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;
    let mut port = Port::new(0xf4); // creating a port is safe but accessing it is not
    unsafe {
        port.write(exit_code as u32);
    } // exit_code should implement the portwrite trait which is done only by u8,u16,u32
}
