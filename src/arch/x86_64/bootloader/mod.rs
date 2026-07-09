/// Bootloader integration module for UEFI and BIOS support
/// This module provides abstractions for both UEFI and legacy BIOS boot modes

pub mod uefi;
pub mod bios;

use core::fmt;

/// Boot information passed by the bootloader
#[derive(Debug, Clone, Copy)]
pub struct BootInfo {
    pub boot_mode: BootMode,
    pub memory_map_addr: u64,
    pub memory_map_size: u64,
    pub framebuffer_addr: u64,
    pub framebuffer_width: u32,
    pub framebuffer_height: u32,
    pub framebuffer_pitch: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BootMode {
    UEFI,
    BIOS,
    Unknown,
}

impl fmt::Display for BootMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BootMode::UEFI => write!(f, "UEFI"),
            BootMode::BIOS => write!(f, "BIOS"),
            BootMode::Unknown => write!(f, "Unknown"),
        }
    }
}

/// Initialize bootloader services
pub fn init() {
    // Detect boot mode and initialize accordingly
    let boot_mode = detect_boot_mode();
    
    match boot_mode {
        BootMode::UEFI => uefi::init(),
        BootMode::BIOS => bios::init(),
        BootMode::Unknown => {
            // Fallback to BIOS mode
            bios::init();
        }
    }
}

/// Detect whether the system was booted via UEFI or BIOS
fn detect_boot_mode() -> BootMode {
    // Check for UEFI by looking at processor state
    // UEFI leaves certain registers in specific states
    unsafe {
        let mut rax: u64;
        asm!("mov {}, rax", out(reg) rax);
        
        // This is a simplified check - real detection would be more complex
        // Check if we're in long mode and have certain UEFI indicators
        if is_uefi_mode() {
            BootMode::UEFI
        } else {
            BootMode::BIOS
        }
    }
}

#[inline]
unsafe fn is_uefi_mode() -> bool {
    // Check CPUID for UEFI indicators
    // This is a simplified version - actual UEFI detection involves
    // checking firmware tables and secure boot state
    true // Placeholder: would need proper detection
}
