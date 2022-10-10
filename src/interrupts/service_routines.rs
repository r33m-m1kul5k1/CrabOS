/* A list of all interrupt service routines */


use x86_64::structures::idt::InterruptStackFrame;
use crate::{logger};



pub extern "x86-interrupt" fn divide_error(stack_frame: InterruptStackFrame) {
    
    logger::debug!("EXCEPTION: DIVIDE ERROR");
    logger::debug!("Stack Frame: {:#X?}", stack_frame);
    panic!();
}

// x86_64::structures::idt::DivergingHandlerFuncWithErrCode
pub extern "x86-interrupt" fn double_fault(stack_frame: InterruptStackFrame, error_code: u64) -> ! {

    loop {}
}

// x86_64::structures::idt::HandlerFuncWithErrCode
pub extern "x86-interrupt" fn general_protection_fault(stack_frame: InterruptStackFrame, error_code: u64) {

}