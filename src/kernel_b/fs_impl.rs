/// Kernel B: Filesystem module wrapper
/// Re-exports existing fs module

pub use crate::fs::*;

pub fn init() {
    crate::fs::init();
}
