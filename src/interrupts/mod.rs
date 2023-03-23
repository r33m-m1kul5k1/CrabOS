//! The interrupt module goal is to manage a minimal interrupt functionalities

use self::gdt::GDT;

pub mod idt;
pub mod gdt;
mod service_routines;

/// Returns the userland code and data selectors
pub fn get_userland_selectors() -> (u16, u16) {
    (GDT.1.user_code.0, GDT.1.user_data.0)
}
