## UART

### [info](https://wiki.osdev.org/UART)

### Printing to the Console

To see the test output on the console, we need to send the data from our kernel to the host system somehow. There are various ways to achieve this, for example, by sending the data over a TCP network interface. However, setting up a networking stack is quite a complex task, so we will choose a simpler solution instead.

## Serial Port

A simple way to send the data is to use the serial port, an old interface standard which is no longer found in modern computers. It is easy to program and QEMU can redirect the bytes sent over serial to the host’s standard output or a file.

The chips implementing a serial interface are called UARTs. There are lots of UART models on x86, but fortunately the only differences between them are some advanced features we don’t need. The common UARTs today are all compatible with the 16550 UART, so we will use that model for our testing framework.

We will use the uart_16550 crate to initialize the UART and send data over the serial port. To add it as a dependency, we update our Cargo.toml and main.rs:

```Rust
# in Cargo.toml

[dependencies]
uart_16550 = "0.2.0"
```

The uart_16550 crate contains a SerialPort struct that represents the UART registers, but we still need to construct an instance of it ourselves. For that, we create a new serial module with the following content:

```Rust
// in src/main.rs

mod serial;
```

```Rust
// in src/serial.rs

use uart_16550::SerialPort;
use spin::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref SERIAL1: Mutex<SerialPort> = {
        let mut serial_port = unsafe { SerialPort::new(0x3F8) };
        serial_port.init();
        Mutex::new(serial_port)
    };
}
```

Like with the VGA text buffer, we use lazy_static and a spinlock to create a static writer instance. By using lazy_static we can ensure that the init method is called exactly once on its first use.

Like the isa-debug-exit device, the UART is programmed using port I/O. Since the UART is more complex, it uses multiple I/O ports for programming different device registers. The unsafe SerialPort::new function expects the address of the first I/O port of the UART as an argument, from which it can calculate the addresses of all needed ports. We’re passing the port address 0x3F8, which is the standard port number for the first serial interface.

To make the serial port easily usable, we add serial_print! and serial_println! macros:

```Rust
// in src/serial.rs

#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) {
    use core::fmt::Write;
    SERIAL1.lock().write_fmt(args).expect("Printing to serial failed");
}

/// Prints to the host through the serial interface.
#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::serial::_print(format_args!($($arg)*));
    };
}

/// Prints to the host through the serial interface, appending a newline.
#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(
        concat!($fmt, "\n"), $($arg)*));
}
```

The implementation is very similar to the implementation of our print and println macros. Since the SerialPort type already implements the fmt::Write trait, we don’t need to provide our own implementation.

Now we can print to the serial interface instead of the VGA text buffer in our test code:

```Rust
// in src/main.rs

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    […]
}

#[test_case]
fn trivial_assertion() {
    serial_print!("trivial assertion... ");
    assert_eq!(1, 1);
    serial_println!("[ok]");
}
```

Note that the serial_println macro lives directly under the root namespace because we used the #[macro_export] attribute, so importing it through use crate::serial::serial_println will not work.

***

***

## VGA Buffer - VGA text mode

The VGA text mode is a simple way to print text to the screen. This is an interface that makes its usage safe and simple by encapsulating all unsafety in a separate module. We also implement support for Rust’s formatting macros.

### The VGA Text Buffer

To print a character to the screen in VGA text mode, one has to write it to the text buffer of the VGA hardware. The VGA text buffer is a two-dimensional array with typically 25 rows and 80 columns, which is directly rendered to the screen. Each array entry describes a single screen character through the following format:

| Bit(s)  | Value          |
|---------|----------------|
| 0-7     |ASCII code point|
| 8-11    |Foreground color|
| 12-14   |Background color|
| 15      |Blink           |

The first byte represents the character that should be printed in the ASCII encoding. To be more specific, it isn’t exactly ASCII, but a character set named code page 437 with some additional characters and slight modifications. For simplicity, we will proceed to call it an ASCII character in this post.

The second byte defines how the character is displayed. The first four bits define the foreground color, the next three bits the background color, and the last bit whether the character should blink. The following colors are available:

#### [The VGA Text Buffer - Table of colors](https://os.phil-opp.com/vga-text-mode/#the-vga-text-buffer)

Bit 4 is the bright bit, which turns, for example, blue into light blue. For the background color, this bit is repurposed as the blink bit.

The VGA text buffer is accessible via memory-mapped I/O to the address 0xb8000. This means that reads and writes to that address don’t access the RAM but directly access the text buffer on the VGA hardware. This means we can read and write it through normal memory operations to that address.

Note that memory-mapped hardware might not support all normal RAM operations. For example, a device could only support byte-wise reads and return junk when a u64 is read. Fortunately, the text buffer supports normal reads and writes, so we don’t have to treat it in a special way.

Now that we know how the VGA buffer works, we can create a Rust module to handle printing:

```Rust
// in src/main.rs
mod vga_buffer;
```

For the content of this module, we create a new src/vga_buffer.rs file. All of the code below goes into our new module (unless specified otherwise).

### Colors

```Rust
/ in src/vga_buffer.rs

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
```

We use a C-like enum here to explicitly specify the number for each color. Because of the repr(u8) attribute, each enum variant is stored as a u8. Actually 4 bits would be sufficient, but Rust doesn’t have a u4 type.

Normally the compiler would issue a warning for each unused variant. By using the #[allow(dead_code)] attribute, we disable these warnings for the Color enum.

By deriving the Copy, Clone, Debug, PartialEq, and Eq traits, we enable copy semantics for the type and make it printable and comparable.

To represent a full color code that specifies foreground and background color, we create a newtype on top of u8:

```Rust
// in src/vga_buffer.rs

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}
```

The ColorCode struct contains the full color byte, containing foreground and background color. Like before, we derive the Copy and Debug traits for it. To ensure that the ColorCode has the exact same data layout as a u8, we use the repr(transparent) attribute.

### Text Buffer

Now we can add structures to represent a screen character and the text buffer:

```Rust
// in src/vga_buffer.rs

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
} 
```

Since the field ordering in default structs is undefined in Rust, we need the repr(C) attribute. It guarantees that the struct’s fields are laid out exactly like in a C struct and thus guarantees the correct field ordering. For the Buffer struct, we use repr(transparent) again to ensure that it has the same memory layout as its single field.

To actually write to screen, we now create a writer type:

```Rust
// in src/vga_buffer.rs

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}
```

The writer will always write to the last line and shift lines up when a line is full (or on \n). The column_position field keeps track of the current position in the last row. The current foreground and background colors are specified by color_code and a reference to the VGA buffer is stored in buffer. Note that we need an explicit lifetime here to tell the compiler how long the reference is valid. The 'static lifetime specifies that the reference is valid for the whole program run time (which is true for the VGA text buffer).


### Printing

Now we can use the Writer to modify the buffer’s characters. First we create a method to write a single ASCII byte:

```Rust 
// in src/vga_buffer.rs

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col] = ScreenChar {
                    ascii_character: byte,
                    color_code,
                };
                self.column_position += 1;
            }
        }
    }

    fn new_line(&mut self) {/* TODO */}
}
```

If the byte is the newline byte \n, the writer does not print anything. Instead, it calls a new_line method, which we’ll implement later. Other bytes get printed to the screen in the second match case.

When printing a byte, the writer checks if the current line is full. In that case, a new_line call is used to wrap the line. Then it writes a new ScreenChar to the buffer at the current position. Finally, the current column position is advanced.

To print whole strings, we can convert them to bytes and print them one-by-one:

```Rust
// in src/vga_buffer.rs

impl Writer {
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // not part of printable ASCII range
                _ => self.write_byte(0xfe),
            }

        }
    }
}
```

The VGA text buffer only supports ASCII and the additional bytes of code page 437. Rust strings are UTF-8 by default, so they might contain bytes that are not supported by the VGA text buffer. We use a match to differentiate printable ASCII bytes (a newline or anything in between a space character and a ~ character) and unprintable bytes. For unprintable bytes, we print a ■ character, which has the hex code 0xfe on the VGA hardware.

example:

```Rust
// in src/vga_buffer.rs

pub fn print_something() {
    let mut writer = Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };

    writer.write_byte(b'H');
    writer.write_string("ello ");
    writer.write_string("Wörld!");
}
```

***
