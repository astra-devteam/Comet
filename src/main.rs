#![no_std]
#![no_main]
#![feature(asm_const)]

extern crate alloc;

use core::panic::PanicInfo;

mod vga;
mod arch;
mod mm;
mod panic_handler;
mod fs;
mod shell;
mod gui;

// Fusion Kernel Architecture modules
mod kernel_a;
mod fusion;
mod kernel_b;
mod system;

use vga::writer::{VgaWriter, Color};
use shell::{Shell, CommandResult};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Initialize VGA console
    let mut vga = VgaWriter::new();
    vga.clear_screen();
    
    vga.write_str("╔════════════════════════════════════════╗\n");
    vga.write_str("║       Welcome to Comet OS              ║\n");
    vga.write_str("║  Fusion Kernel Architecture - v0.2.0   ║\n");
    vga.write_str("╚════════════════════════════════════════╝\n\n");
    
    vga.write_str("⸻ Boot Sequence ⸻\n\n");
    
    // Fusion Kernel Architecture Boot Sequence
    // _start() → Kernel A → Fusion → Kernel B → System Distribution → Shell
    
    // Initialize Kernel A (Core components)
    kernel_a::init(&mut vga);
    
    // Initialize Fusion (Object/Handle Management)
    fusion::init(&mut vga);
    
    // Initialize Kernel B (High-level Services)
    kernel_b::init(&mut vga);
    
    // Initialize System Distribution Layer
    system::init(&mut vga);
    
    vga.write_str("System initialization complete. Starting shell...\n");
    vga.write_str("Type 'help' for available commands\n");
    vga.write_str("─".repeat(40));
    vga.write_str("\n\n");
    
    // Start shell
    shell_main();
}

fn shell_main() -> ! {
    let mut shell = Shell::new();
    let mut vga = VgaWriter::new();
    
    loop {
        shell.display_prompt();
        
        // Simple simulation of command input
        // In a real system, this would wait for keyboard input
        simulate_command_input(&mut shell, &mut vga);
    }
}

fn simulate_command_input(shell: &mut Shell, vga: &mut VgaWriter) {
    // Display help text on first run
    vga.write_str("help\n");
    
    let cmd = shell.parse_command("help");
    let result = shell.execute_command(cmd);
    
    match result {
        CommandResult::Success(msg) => {
            if !msg.is_empty() {
                vga.write_str(&msg);
            }
        }
        CommandResult::Error(err) => {
            vga.set_color(Color::Red, Color::Black);
            vga.write_str("Error: ");
            vga.set_color(Color::White, Color::Black);
            vga.write_str(&err);
            vga.write_str("\n");
        }
        CommandResult::Exit => {
            vga.write_str("Goodbye!\n");
            hlt_loop();
        }
    }
    
    vga.write_str("\n");
}

pub fn hlt_loop() -> ! {
    loop {
        #[cfg(target_arch = "x86_64")]
        unsafe {
            asm!("hlt");
        }
    }
}
