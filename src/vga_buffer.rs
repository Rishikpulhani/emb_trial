use core::fmt::{Error, Write};
use core::str::{self};
use volatile::Volatile;

#[allow(dead_code)]
//#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)] // ensures that the enum is represented as a u8 in memoryand is directly convertiable to u8 using the as keyword
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    LightMagenta = 13,
    Yellow = 14,
    White = 15,
}
//attribute byte
#[repr(transparent)]
// this ensures that rust does not use its defualt memory layoout by adding padding as that can cause issues with the memory layout of the struct and in direct memory accesswe cannot use this uncertain behaviuor - tramsparent means that the struct will have the same memory layout as the inner type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ColorCode(u8);
impl ColorCode {
    fn new(fg: Color, bg: Color) -> ColorCode {
        ColorCode(((bg as u8) << 4) | (fg as u8)) // need to convert to u8 as << only works on u8 not enum
    }
}
//character = attribute byte + character value (ascii value)
//#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(Clone, Copy)]
#[repr(C)]
struct ScreenChar {
    ascii_char: u8,
    color_code: ColorCode,
}
// buffer layout
// the buffer is a 2d array of ScreenChar where each element is a ScreenChar
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

// entire buffer table
#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

// an entity to write to the vga buffer, we use this to store info about the place where we want to write a character
pub struct Writer {
    colomn_position: usize,
    color_code: ColorCode,       // tells both the front and background color
    buffer: &'static mut Buffer, // to access the place where we want to write character
}
// now only require the function to tell the ascii value to be written the rest is in the writer object
impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.newline(),
            byte => {
                if self.colomn_position >= BUFFER_WIDTH {
                    self.newline();
                }
                let row = BUFFER_HEIGHT - 1; // we always write value at the bottom and move things up, we chnage tjhe value iof the buffer_height
                let col = self.colomn_position;
                let color_code = self.color_code; // this requires ColorCode to implement copy else it will move the value
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_char: byte,
                    color_code,
                });
                // this ensures that the compiler will not optimise this write
                self.colomn_position += 1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // as rust strings are utf 8 and here we deal with extended ascii so out of range is possible
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }

    fn newline(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                self.buffer.chars[row - 1][col].write(self.buffer.chars[row][col].read());
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.colomn_position = 0;
    }
    fn clear_row(&mut self, row: usize) {
        let character = ScreenChar {
            ascii_char: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_HEIGHT {
            self.buffer.chars[row][col].write(character);
        }
    }
}
pub fn print_something() {
    let mut writer = Writer {
        colomn_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };
    writer.write_byte(b'H');
    writer.write_string("ello ");
    write!(writer, "The numbers are {} and {}", 42, 1.0 / 3.0).unwrap();
}

impl Write for Writer {
    fn write_str(&mut self, s: &str) -> Result<(), Error> {
        self.write_string(s);
        Ok(())
    }
}
