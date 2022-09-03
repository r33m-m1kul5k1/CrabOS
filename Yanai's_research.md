***

## Booting Process

1. Power on.
2. CPU will move to BIOS (Basic Input Output System) in ROM (Read Only Memory).
3. BIOS will be executed (POST - Power On Self Test) : all the hardware will be tested.
This is a working condition to the hardware which is related to the CPU:
If the post is successful, then the hardware is working.
If the hardware is not working, the booting process will automatically stop.
4. BIOS will load MBR (Master Boot Record) to RAM (Random Access Memory).
5. MBR will load bootloader to RAM.
6. Bootloader will load OS to RAM.
7. ow, It starts executing the OS, and the control will move to the OS.

Hard Booting - Power on.
Soft Booting - Restart.

### [Booting Process](https://www.youtube.com/watch?v=bDsTwHIqs2g)

***

***

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

## VGA Buffer

```Rust
```

***

