pub mod interrupts;
pub mod gdt;
pub mod bootloader;

pub fn init() {
    gdt::init();
    interrupts::init_idt();
    bootloader::init();
}
