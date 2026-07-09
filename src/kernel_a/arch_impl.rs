/// Kernel A: Architecture module wrapper
/// Re-exports existing arch module

pub use crate::arch::*;

pub fn init() {
    // CPU initialization is delegated to existing arch module
    unsafe {
        crate::arch::x86_64::init();
        crate::arch::x86_64::interrupts::init_idt();
    }
}
