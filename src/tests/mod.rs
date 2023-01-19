use crate::serial_println;
use crate::panic::{QemuExitCode, exit_qemu};
pub trait Testable {
    fn run(&self) -> ();
}

/// Defines a test function which have a run method
impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_println!("{}...", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

/// Run the array of #[test_case] functions, and implement for them the `Testable` trait
pub fn runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());

    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}


