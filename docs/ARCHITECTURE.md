# Comet OS Architecture

## Overview

Comet is a minimal x86_64 kernel demonstrating core OS concepts:
- Boot sequence and CPU initialization
- Interrupt handling (IDT)
- Virtual memory management (paging)
- Heap allocation
- VGA console for debugging

## Boot Sequence

```
[BIOS/UEFI] 
    ↓
[Bootloader] → Load kernel to 0x100000
    ↓
[_start()] → Initialize CPU
    ↓
[Kernel main] → Initialize subsystems
    ↓
[hlt_loop] → Idle
```

## CPU Initialization (GDT/IDT)

### Global Descriptor Table (GDT)
- Kernel Code Segment
- Task State Segment (TSS)
- Double fault handling stack

### Interrupt Descriptor Table (IDT)
Handles:
- **Exceptions**: Breakpoint, Page Fault, General Protection, Invalid Opcode, Divide Error
- **Future**: IRQs for hardware interrupts

## Memory Layout

```
0xFFFF_FFFF_FFFF_FFFF ├─────────────────┐
                       │ Kernel Heap     │
                       │ (512 MiB)       │
0xFFFF_FFFF_E000_0000  ├─────────────────┤
                       │ Kernel Text     │
                       │ (2 GiB)         │
0xFFFF_FFFF_8000_0000  ├─────────────────┤
                       │ ...             │
0x0000_4444_0000_0000  ├─────────────────┤
                       │ User Space      │
0x0000_0000_0000_0000  └─────────────────┘
```

## VGA Console

Direct memory-mapped VGA buffer at 0xB8000:
- 80 × 25 text mode
- Each character: 1 byte ASCII + 1 byte color
- Direct memory writes for immediate display

## Future Work

- [ ] Page frame allocator
- [ ] Dynamic paging
- [ ] Heap allocator improvements
- [ ] Interrupt request (IRQ) handlers
- [ ] Keyboard/input driver
- [ ] Disk I/O driver
- [ ] Process/task management
- [ ] Scheduling
- [ ] System calls

## Building and Running

```bash
# Build
cargo build --release

# Create bootable image
cargo bootimage --release

# Run in QEMU
qemu-system-x86_64 -drive format=raw,file=target/x86_64-comet-os/release/boot-image.bin
```

## References

- [OSDev.org](https://wiki.osdev.org/)
- [x86_64 Crate](https://docs.rs/x86_64/)
- [Bootloader Crate](https://docs.rs/bootloader/)
