/// README for Comet OS Modules
///
/// ## Overview
/// Comet OS is a minimal x86_64 kernel with support for UEFI and BIOS bootloaders.
/// It includes:
/// - Bootloader abstraction (UEFI/BIOS)
/// - Virtual filesystem
/// - Command-line shell interface
/// - Simple GUI framework
/// - Interrupt handling
/// - GDT/IDT management
///
/// ## Module Structure
///
/// ### `src/arch/x86_64/`
/// - `bootloader/` - UEFI and BIOS bootloader support
///   - `mod.rs` - Boot mode detection
///   - `uefi.rs` - UEFI-specific initialization
///   - `bios.rs` - BIOS/Legacy BIOS support
/// - `gdt.rs` - Global Descriptor Table setup
/// - `interrupts.rs` - Interrupt descriptor table and handlers
/// - `mod.rs` - x86_64 architecture module
///
/// ### `src/fs/`
/// - `mod.rs` - Filesystem types and initialization
/// - `inode.rs` - Inode management
/// - `vfs.rs` - Virtual filesystem implementation
///
/// ### `src/shell/`
/// - `mod.rs` - Shell structure and prompt handling
/// - `parser.rs` - Command parser
/// - `commands.rs` - Built-in command implementations
///
/// ### `src/gui/`
/// - `mod.rs` - GUI manager and coordinate system
/// - `window.rs` - Window management
/// - `widget.rs` - Widget types (label, button, textbox, progress bar)
///
/// ## Built-in Shell Commands
/// - `ls` - List directory contents
/// - `cd` - Change directory
/// - `pwd` - Print working directory
/// - `echo` - Display text
/// - `uname` - Print system information
/// - `whoami` - Display current user
/// - `clear` - Clear screen
/// - `help` - Display command help
/// - `exit`/`quit` - Exit shell
///
/// ## Bootloader Support
/// The kernel automatically detects the boot mode and initializes accordingly:
/// - **UEFI**: Uses UEFI runtime services for firmware interaction
/// - **BIOS**: Legacy BIOS mode with INT 0x15 E820 memory detection
///
/// ## Building
/// ```bash
/// cargo build --target x86_64-bootloader
/// ```
///
/// ## Future Enhancements
/// - Process management and scheduling
/// - Network stack
/// - Advanced memory management
/// - Disk I/O drivers
/// - Extended filesystem support (ext2, FAT32)
/// - Mouse support for GUI
