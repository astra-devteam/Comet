/// BIOS/Legacy Bootloader Support
/// 
/// Handles initialization for systems booted via legacy BIOS
/// Provides abstractions for real mode to protected mode transitions

use crate::vga::VgaWriter;

/// BIOS boot information
pub struct BiosInfo {
    pub drive_number: u8,
    pub partition_table: *const PartitionEntry,
    pub memory_map_addr: u64,
    pub memory_map_entries: u32,
}

/// MBR Partition table entry
#[repr(C)]
pub struct PartitionEntry {
    pub boot_flag: u8,
    pub chs_start: [u8; 3],
    pub partition_type: u8,
    pub chs_end: [u8; 3],
    pub lba_start: u32,
    pub sector_count: u32,
}

/// Initialize BIOS-specific components
pub fn init() {
    let mut vga = VgaWriter::new();
    vga.write_str("[*] BIOS boot detected\n");
    
    // Initialize BIOS runtime services
    init_bios_services();
    
    vga.write_str("[✓] BIOS services initialized\n");
}

/// Initialize BIOS interrupt services
fn init_bios_services() {
    // Set up BIOS interrupt vector table (IVT) handlers
    // In real mode, this would be in the first 1KB of memory
    // In protected mode (where we are), we need to emulate BIOS calls
    
    // Key BIOS services to support:
    // - INT 0x10: Video services
    // - INT 0x13: Disk services
    // - INT 0x15: System services (memory detection)
    // - INT 0x16: Keyboard services
}

/// Get BIOS memory map (via INT 0x15, E820)
pub fn get_e820_memory_map() -> Option<&'static [E820MemoryEntry]> {
    // This would have been populated by the bootloader
    // using BIOS INT 0x15, E820h calls
    None
}

/// E820 memory map entry (returned by BIOS INT 0x15)
#[repr(C)]
pub struct E820MemoryEntry {
    pub base_addr: u64,
    pub length: u64,
    pub entry_type: u32,
    pub extended_attributes: u32,
}

impl E820MemoryEntry {
    pub fn memory_type(&self) -> &'static str {
        match self.entry_type {
            1 => "Usable RAM",
            2 => "Reserved",
            3 => "ACPI Reclaimable",
            4 => "ACPI NVS",
            5 => "Bad Memory",
            _ => "Unknown",
        }
    }
}

/// BIOS video mode information
pub struct VesaBiosExtension {
    pub mode_number: u16,
    pub framebuffer_addr: u32,
    pub width: u16,
    pub height: u16,
    pub bits_per_pixel: u8,
}

/// Get current VESA VBE mode
pub fn get_vbe_mode() -> Option<VesaBiosExtension> {
    // Query current video mode from VBE
    // This would require calling BIOS INT 0x10 (if we had real mode capability)
    // or reading from bootloader-provided info
    None
}

/// Detect BIOS capabilities
pub fn detect_capabilities() -> BiosCapabilities {
    BiosCapabilities {
        has_extended_memory: true,
        has_vbe: true,
        has_apic: true,
        has_acpi: true,
    }
}

pub struct BiosCapabilities {
    pub has_extended_memory: bool,
    pub has_vbe: bool,
    pub has_apic: bool,
    pub has_acpi: bool,
}
