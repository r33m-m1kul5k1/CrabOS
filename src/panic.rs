use crate::serial_println;
pub use core::panic::PanicInfo;

const ISA_DEBUG_EXIT_PORT: u16 = 0xf4;
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

/// Exits qemu by writing the exit code to the debug exit port
pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(ISA_DEBUG_EXIT_PORT);
        port.write(exit_code as u32);
    }
}

/// Halts the CPU infinitely
pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

/// Panic handler for the kernel
pub fn kernel_panic(info: &PanicInfo) -> ! {
    serial_println!("[KERNEL PANIC]");
    serial_println!("Error: {}", info);
    exit_qemu(QemuExitCode::Failed);
    hlt_loop();
}

/// Panic handler for integration testing
pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    hlt_loop();
}

/// Panic handler that expects to be called
pub fn test_should_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[Success]\n");
    serial_println!("Panic info: {}\n", info);
    exit_qemu(QemuExitCode::Success);
    hlt_loop();
}
