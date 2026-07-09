use core::panic::PanicInfo;
use crate::vga::{VgaWriter, Color};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let mut vga = VgaWriter::new();
    vga.set_color(Color::Red, Color::Black);
    
    vga.write_str("\n╔════════════════════════════════════════╗\n");
    vga.write_str("║            KERNEL PANIC               ║\n");
    vga.write_str("╚════════════════════════════════════════╝\n");
    
    if let Some(location) = info.location() {
        vga.set_color(Color::Yellow, Color::Black);
        vga.write_str("Location: ");
        vga.set_color(Color::White, Color::Black);
        vga.write_str(location.file());
        vga.write_str(":");
        // Simple number to string conversion
        let line = location.line();
        write_number(&mut vga, line as usize);
        vga.write_str("\n");
    }

    if let Some(msg) = info.message() {
        vga.set_color(Color::Yellow, Color::Black);
        vga.write_str("Message: ");
        vga.set_color(Color::White, Color::Black);
        // Note: fmt::Arguments doesn't implement Display, so we can't directly print it
        vga.write_str("(see debug output)\n");
    }

    vga.set_color(Color::Red, Color::Black);
    vga.write_str("\nSystem halted.\n");

    loop {
        #[cfg(target_arch = "x86_64")]
        unsafe {
            asm!("hlt");
        }
    }
}

fn write_number(vga: &mut VgaWriter, mut num: usize) {
    if num == 0 {
        vga.write_byte(b'0');
        return;
    }

    let mut digits = [0u8; 20];
    let mut len = 0;

    while num > 0 {
        digits[len] = (num % 10) as u8 + b'0';
        num /= 10;
        len += 1;
    }

    for i in (0..len).rev() {
        vga.write_byte(digits[i]);
    }
}
