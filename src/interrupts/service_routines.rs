//! A list of all interrupt service routines

use crate::log::debug;
use x86_64::structures::idt::{InterruptStackFrame, PageFaultErrorCode};

/// A page fault occures when:
///
/// 1. The page entry is not present
/// 2. Attempting to load to the TLB a non-executable page
/// 3. Protection checks
/// 4. If a reserved bit is set to 1.
/// The address pushed to the stack points to the faulty instruction.
pub extern "x86-interrupt" fn page_fault(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    debug!("EXCEPTION: page fault");
    debug!("Error code: {:#X?}", error_code);
    debug!("Stack frame: {:#X?}", stack_frame);
    panic!();
}
/// A double fault (#DF) exception can occur
/// when a second exception occurs during the handling of a prior (first) exception or interrupt handler.
pub extern "x86-interrupt" fn double_fault(stack_frame: InterruptStackFrame, error_code: u64) -> ! {
    debug!("EXCEPTION: double fault");
    debug!("Error code: {:#X?}", error_code);
    debug!("Stack frame: {:#X?}", stack_frame);
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
    debug!("EXCEPTION: general protection fault");
    debug!("Error code: {:#X?}", error_code);
    debug!("Stack frame: {:#X?}", stack_frame);
    panic!();
}
