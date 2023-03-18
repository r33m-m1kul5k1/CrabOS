//! This module writes to the VGA buffer in text mode.

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;
const ADDRESS: usize = 0xb8000;
const SPACE_ASCII: u8 = 0x20;
const TILDA_ASCII: u8 = 0x7e;
const NOT_IN_ASCII_RANGE: u8 = 0xfe;

use core::fmt::{self, Arguments};
use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(
        Writer {
            column_position: 0,
            row_position: 0,
            color_code: ColorCode::new(Color::White, Color::Black),
            buffer: unsafe { &mut *(ADDRESS as *mut Buffer) },
        }
    );
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)] 
#[repr(u8)]
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
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
/// ColorCode struct contains the full color byte, containing foreground and background color. 
struct ColorCode(u8); 

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

#[repr(transparent)]
/// An 2d array representing the vga buffer
struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

/// This struct writes to a given buffer in text mode
pub struct Writer {

    column_position: usize,
    row_position: usize,
    // Writer's theme
    color_code: ColorCode,
    /// The vga buffer
    buffer: &'static mut Buffer,
}

impl Writer {

    /// write one byte to the buffer, if new line go to the next row.
    fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                self.buffer.chars[self.row_position][self.column_position] = ScreenChar {
                    ascii_character: byte,
                    color_code: self.color_code,
                };
                self.column_position += 1;
            }
        }
    }
    
    /// Takes the writer to the start of the row below the current one.
    fn new_line(&mut self) {
        self.row_position += 1;
        self.column_position = 0;
    }

    /// Writes every ASCII character to the screen
    fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // here we can add backspace operation (0x8)
                SPACE_ASCII..=TILDA_ASCII | b'\n' => self.write_byte(byte),

                // not part of printable ASCII range
                _ => self.write_byte(NOT_IN_ASCII_RANGE),
            }

        }
    }

    /// Set the writer with a new color code.
    pub fn set_writer_theme(&mut self, foreground: Color, background: Color) {
        self.color_code = ColorCode::new(foreground, background);
    }
    
}

impl fmt::Write for Writer {
    
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

/// private print for macros
pub fn _print(args: Arguments) {
    
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}