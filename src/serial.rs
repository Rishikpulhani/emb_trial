use core::fmt::{self, Write};
use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::SerialPort;

lazy_static! {
    pub static ref SERIAL1: Mutex<SerialPort> = {
        let mut serial_port = unsafe{SerialPort::new(0x3F8)};// here we usethe interiro mutability of mutex
        serial_port.init();
        Mutex::new(serial_port)
    };
}

// adding support for println macro
// the serial printingis similar to the vga buffer also the serial port implements the Write trait so unlike the vga buffer no need of a custom implmementation
// this only sends the output of qemu to the serial port but we need to specify the final destination here 
#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!( // this is more efficient than the println as here is a special condition that there wont be a variable but only format strings sowe use concat which converts it to a single format string at compile time
        concat!($fmt, "\n"), $($arg)*));
}
#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => ($crate::serial::_print(format_args!($($arg)*)));
}
#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    SERIAL1
        .lock()
        .write_fmt(args)
        .expect("Prinitng to serial failed");
}
