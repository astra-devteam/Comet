use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::vga::{VgaWriter, Color};

static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();

pub fn init_idt() {
    unsafe {
        // Setup exception handlers
        IDT.breakpoint.set_handler_fn(breakpoint_handler);
        IDT.double_fault.set_handler_fn(double_fault_handler);
        IDT.page_fault.set_handler_fn(page_fault_handler);
        IDT.general_protection_fault.set_handler_fn(general_protection_fault_handler);
        IDT.invalid_opcode.set_handler_fn(invalid_opcode_handler);
        IDT.divide_error.set_handler_fn(divide_error_handler);
        
        IDT.load();
    }
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    let mut vga = VgaWriter::new();
    vga.set_color(Color::Yellow, Color::Black);
    vga.write_str("\n[!] Breakpoint Exception\n");
    vga.write_str("Stack Frame: ");
    vga.set_color(Color::White, Color::Black);
    vga.write_str("IP=0x");
    // Would need to format the address
    vga.write_str("\n");
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    let mut vga = VgaWriter::new();
    vga.set_color(Color::Red, Color::Black);
    vga.write_str("\n╔════════════════════════════════════════╗\n");
    vga.write_str("║        DOUBLE FAULT EXCEPTION         ║\n");
    vga.write_str("╚════════════════════════════════════════╝\n");
    vga.set_color(Color::White, Color::Black);
    vga.write_str("An unrecoverable exception occurred.\n");
    
    loop {
        #[cfg(target_arch = "x86_64")]
        unsafe {
            asm!("hlt");
        }
    }
}

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: x86_64::structures::idt::PageFaultErrorCode,
) {
    let mut vga = VgaWriter::new();
    vga.set_color(Color::Red, Color::Black);
    vga.write_str("\n[!] Page Fault Exception\n");
    vga.set_color(Color::White, Color::Black);
}

extern "x86-interrupt" fn general_protection_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) {
    let mut vga = VgaWriter::new();
    vga.set_color(Color::Red, Color::Black);
    vga.write_str("\n[!] General Protection Fault\n");
    vga.set_color(Color::White, Color::Black);
}

extern "x86-interrupt" fn invalid_opcode_handler(stack_frame: InterruptStackFrame) {
    let mut vga = VgaWriter::new();
    vga.set_color(Color::Red, Color::Black);
    vga.write_str("\n[!] Invalid Opcode Exception\n");
    vga.set_color(Color::White, Color::Black);
}

extern "x86-interrupt" fn divide_error_handler(stack_frame: InterruptStackFrame) {
    let mut vga = VgaWriter::new();
    vga.set_color(Color::Red, Color::Black);
    vga.write_str("\n[!] Divide Error Exception\n");
    vga.set_color(Color::White, Color::Black);
}
