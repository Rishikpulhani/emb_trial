#![no_std]
#![no_main]
use core::panic::PanicInfo;
mod vga_buffer;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
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
    vga_buffer::print_something();
    loop {}
}
