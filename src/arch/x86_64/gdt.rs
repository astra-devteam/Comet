use x86_64::structures::gdt::{GlobalDescriptorTable, Descriptor};
use x86_64::structures::tss::TaskStateSegment;
use x86_64::VirtAddr;

const DOUBLE_FAULT_IST_INDEX: u16 = 0;

static mut GDT: GlobalDescriptorTable = GlobalDescriptorTable::new();
static mut TSS: TaskStateSegment = TaskStateSegment::new();

pub fn init() {
    unsafe {
        // Initialize TSS
        let stack_start = VirtAddr::new(0x4000_0000);
        let stack_end = stack_start + 0x1000;
        TSS.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = stack_end;

        // Load GDT
        GDT.add_entry(Descriptor::kernel_code_segment());
        GDT.add_entry(Descriptor::tss_segment(&TSS));
        GDT.load();
    }
}
