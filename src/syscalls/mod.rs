//! This module defines the syscall inteface and it's dispatcher
use core::arch::asm;

macro_rules! wrap_syscall_handler {
    ($fn:ident => $wrapper:ident) => {
        #[naked]
        pub unsafe extern "C" fn $wrapper() {
            asm!(
                "call {}", 
                "iretq",
                sym $fn,
                options(noreturn)
            );
        }
    };
}
wrap_syscall_handler!(syscall_handler => wrapped_syscall_handler);


fn syscall_handler() {
    
}

// fn dispatcher() {}
