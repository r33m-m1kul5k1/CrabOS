//! This module defines the syscall inteface and it's dispatcher
use core::arch::asm;

use log::{debug, info};

use crate::hardware::rdmsr;
use crate::interrupts::{get_kernel_selectors, get_user_selectors};
use crate::memory::as_addr;
use crate::wrmsr;
mod services;

const IA32_EFER_MSR: u64 = 0xC0000080;
const IA32_STAR_MSR: u64 = 0xC0000081;
const IA32_LSTAR_MSR: u64 = 0xC0000082;
const IA32_FMASK_MSR: u64 = 0xC0000084;
const INTERRUPT_ENABLE_FLAG: u64 = 1 << 9;
const SYSCALL_ENABLE_EFER: u64 = 1 << 0;

const MASK_32_HIGH: u64 = 0xFFFFFFFF;

pub fn init() {
    let (cs, _) = get_kernel_selectors();
    let (_, ss) = get_user_selectors();
    // changes the high bytes of IA32_STAR msr to point to the selector as mentioned in https://www.felixcloutier.com/x86/syscall.html
    let star_value =
        (((ss as u64 - 8) << 16) | cs as u64) << 32 | (rdmsr(IA32_STAR_MSR) & MASK_32_HIGH);

    wrmsr!(IA32_STAR_MSR, star_value);
    wrmsr!(IA32_LSTAR_MSR, as_addr(&syscall_handler));
    wrmsr!(IA32_FMASK_MSR, INTERRUPT_ENABLE_FLAG);
    wrmsr!(IA32_EFER_MSR, rdmsr(IA32_EFER_MSR) | SYSCALL_ENABLE_EFER);
    
    
    debug!("IA32_STAR: {:#x}", rdmsr(IA32_STAR_MSR));
    debug!("IA32_LSTAR: {:#x}", rdmsr(IA32_LSTAR_MSR));
    debug!("IA32_FMASK: {:#x}", rdmsr(IA32_FMASK_MSR));
    debug!("IA32_EFER: {:#b}", rdmsr(IA32_EFER_MSR));
    info!("syscalls initialized");
}

#[naked]
#[no_mangle]
pub extern "C" fn syscall_handler() {
    unsafe { asm!("hlt", options(noreturn)) };
}

// fn dispatcher() {}
