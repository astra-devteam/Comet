/// Kernel B: High-level kernel components
/// Contains: fs, gui, network (future)

pub mod fs_impl;
pub mod gui_impl;

use crate::vga::writer::VgaWriter;

pub fn init(vga: &mut VgaWriter) {
    vga.write_str("[*] Initializing Kernel B (High-level Services)...\n");
    
    // Initialize filesystem
    fs_impl::init();
    vga.write_str("[✓] Filesystem initialized\n");
    
    // Initialize GUI
    gui_impl::init();
    vga.write_str("[✓] GUI initialized\n");
    
    vga.write_str("[✓] Kernel B ready\n\n");
}
