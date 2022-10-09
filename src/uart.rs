// the uart module goal is to override the `println!` macro with a uart print

use uart_16550::SerialPort;

const SERIAL_IO_PORT: u16 = 0x3F8;

/// Returns a serial port for printing
pub fn init_uart() -> SerialPort {
    let mut serial_port = unsafe { SerialPort::new(SERIAL_IO_PORT)};
    serial_port.init();

    serial_port.send(65);

    serial_port
}

