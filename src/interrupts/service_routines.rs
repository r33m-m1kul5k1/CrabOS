/* A list of all interrupt service routines */

use crate::logger;
use x86_64::structures::idt::InterruptStackFrame;

/// A double fault (#DF) exception can occur
/// when a second exception occurs during the handling of a prior (first) exception or interrupt handler.
pub extern "x86-interrupt" fn double_fault(stack_frame: InterruptStackFrame, error_code: u64) -> ! {
    logger::debug!("EXCEPTION: Double Fault");
    logger::debug!("Error code: {:#X?}", error_code);
    logger::debug!("Stack Frame: {:#X?}", stack_frame);
    panic!();
}

/// A general protection fault (#GP) can occur in various situations. Common causes include:
/// 
/// 1. Executing a privileged instruction while CPL > 0.
/// 2. Writing a 1 into any register field that is reserved, must be zero (MBZ).
/// 3. Attempting to execute an SSE instruction specifying an unaligned memory operand.
/// 4. Loading a non-canonical base address into the GDTR or IDTR.
/// 5. Using WRMSR to write a read-only MSR.
/// 6. Any long-mode consistency-check violation.
pub extern "x86-interrupt" fn general_protection_fault(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) {
    logger::debug!("EXCEPTION: General Protection Fault");
    logger::debug!("Error code: {:#X?}", error_code);
    logger::debug!("Stack Frame: {:#X?}", stack_frame);
    panic!();
}
