/// Kernel A: Core kernel components
/// Contains: arch, mm, interrupt, scheduler

pub mod arch_impl;
pub mod mm_impl;

use crate::vga::writer::VgaWriter;

pub fn init(vga: &mut VgaWriter) {
    vga.write_str("[*] Initializing Kernel A (Core)...\n");
    
    // Initialize architecture (CPU, GDT, etc.)
    arch_impl::init();
    vga.write_str("[✓] Architecture initialized\n");
    
    // Initialize memory management
    mm_impl::init();
    vga.write_str("[✓] Memory management initialized\n");
    
    vga.write_str("[✓] Kernel A ready\n\n");
}
