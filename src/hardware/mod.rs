//! defines hardware instruction interface

use core::arch::asm;

#[macro_export]
macro_rules! wrmsr {
    ($msr:expr, $value:expr) => {
        unsafe { asm!("wrmsr", in("edx") $value >> 32, in("eax") $value & 0xFFFFFFFF, in("ecx") $msr) };
    };
}

pub fn rdmsr(msr: u64) -> u64 {
    let mut value_low: u32;
    let mut value_high: u32;
    unsafe { asm!("wrmsr", out("edx") value_high, out("eax") value_low, in("ecx") msr) };

    return ((value_high as u64) << 32) | value_low as u64;
}
