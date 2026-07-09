# Comet OS - Modular Kernel Architecture

## Overview

Comet OS is a minimal but feature-rich x86_64 kernel with support for both UEFI and BIOS bootloaders. It provides:

- **Bootloader Abstraction** - Automatic detection and initialization for UEFI/BIOS
- **Virtual Filesystem** - Inode-based filesystem with directory support
- **Command-Line Shell** - Interactive CLI with built-in commands
- **GUI Framework** - Simple windowed GUI with widgets
- **CPU Management** - GDT, IDT, and interrupt handling

## Module Structure

### Architecture (`src/arch/x86_64/`)

#### Bootloader Support (`bootloader/`)
- **`mod.rs`** - Boot mode detection and initialization routing
- **`uefi.rs`** - UEFI firmware services integration
- **`bios.rs`** - Legacy BIOS mode initialization

#### CPU Management
- **`gdt.rs`** - Global Descriptor Table with TSS setup
- **`interrupts.rs`** - Interrupt descriptor table and exception handlers

### Filesystem (`src/fs/`)

- **`mod.rs`** - File types, permissions, and metadata
- **`inode.rs`** - Inode structures and management
- **`vfs.rs`** - Virtual filesystem with directory operations

### Shell (`src/shell/`)

- **`mod.rs`** - Shell prompt, command execution, and directory navigation
- **`parser.rs`** - Command line parsing and tokenization
- **`commands.rs`** - Built-in command implementations

### GUI (`src/gui/`)

- **`mod.rs`** - GUI manager, coordinate system, and window management
- **`window.rs`** - Window rendering with title bars and borders
- **`widget.rs`** - Widget types: labels, buttons, textboxes, progress bars

## Built-in Shell Commands

| Command | Description |
|---------|-------------|
| `ls` | List directory contents |
| `cd [path]` | Change current directory |
| `pwd` | Print working directory |
| `echo [text...]` | Display text output |
| `uname` | Print system information |
| `whoami` | Display current user |
| `clear` | Clear the screen |
| `help` | Show command reference |
| `exit` / `quit` | Exit the shell |

## Bootloader Support

### UEFI Mode
- Accesses UEFI runtime services
- Memory map via EFI system table
- Framebuffer information from firmware

### BIOS Mode  
- INT 0x15 E820 memory detection
- MBR partition table support
- VESA VBE video mode detection

Automatic detection switches between modes at startup.

## GUI Components

### Window System
- Title bar with focus indication
- Unicode box-drawing borders
- Widget container and rendering

### Widget Types
- **Label** - Static text display
- **Button** - Clickable interactive element
- **TextBox** - User input field
- **ProgressBar** - Visual progress indication

## File Type Support

- **Regular Files** - Standard data files
- **Directories** - File container with entries
- **Symbolic Links** - File references
- **Device Files** - Special device nodes

## Permissions Model

Unix-style permissions with owner/group/other categories:
```
(owner_read, owner_write, owner_execute,
 group_read, group_write, group_execute,
 other_read, other_write, other_execute)
```

## Building and Running

```bash
# Build for x86_64 with bootloader
cargo build --target x86_64-bootloader

# Run in QEMU
qemu-system-x86_64 -kernel target/x86_64-bootloader/debug/comet

# Run with UEFI firmware simulation
qemu-system-x86_64 -bios /usr/share/qemu/OVMF.fd -kernel target/x86_64-bootloader/debug/comet
```

## Architecture Highlights

### Memory Layout
- Kernel code: Mapped at high addresses
- Heap: Managed by global allocator
- Stack: Per-CPU with guard pages
- Physical memory: Mapped via bootloader page tables

### Interrupt Handling
- Double fault handler with dedicated stack
- Page fault detection and reporting
- General protection fault handling
- Invalid opcode detection
- Divide-by-zero handling

### Color Support
VGA text mode with 16 colors:
- Black, Blue, Green, Cyan, Red, Magenta, Brown, White
- Bright variants for better visibility

## Future Enhancements

- [ ] Process management and scheduling
- [ ] Virtual memory and paging
- [ ] Network stack (TCP/IP)
- [ ] Advanced filesystem (ext2, FAT32)
- [ ] Device drivers
- [ ] Mouse support for GUI
- [ ] System call interface
- [ ] User-space programs
- [ ] Disk I/O subsystem
- [ ] Inter-process communication

## Performance Considerations

- Minimal overhead for bootloader abstraction
- Lock-free VFS operations where possible
- Efficient command parsing
- Direct VGA memory access for display

## Security Features

- CPU privilege levels enforcement (Ring 0/3)
- Stack canaries in exception handlers
- Memory protection via paging
- Interrupt vector validation

## Testing

```bash
# Run tests
cargo test --target x86_64-bootloader

# Test components individually
cargo test --lib shell
cargo test --lib fs
cargo test --lib gui
```

---

**Version**: 0.1.0  
**Architecture**: x86_64  
**License**: MIT
