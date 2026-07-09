# Comet OS

A minimal x86_64 operating system written in Rust, designed for educational purposes and as a foundation for exploring OS kernel development.

## Features

- **x86_64 Architecture**: 64-bit x86 processor support
- **Rust-based Kernel**: Memory safety without garbage collection
- **Bootloader**: Custom bootloader for x86_64 UEFI systems
- **Memory Management**: Paging and heap allocation
- **Interrupt Handling**: IDT setup and interrupt vectors
- **VGA Console**: Basic text output for debugging

## Building

### Requirements

- Rust nightly toolchain
- `x86_64-unknown-none` target
- QEMU (for testing)
- GNU Binutils (objcopy)

### Setup

```bash
# Install Rust nightly
rustup install nightly
rustup override set nightly

# Add target
rustup target add x86_64-unknown-none --toolchain nightly

# Install bootimage tool
cargo install bootimage

# Add LLVM tools
cargo install llvm-tools-embedded
```

### Compile

```bash
# Build the OS image
cargo build --release

# Create bootable image
cargo bootimage --release
```

### Run

```bash
# Run in QEMU
qemu-system-x86_64 -drive format=raw,file=target/x86_64-comet-os/release/boot-image.bin
```

## Project Structure

```
comet/
  src/
    main.rs           # Kernel entry point
    lib.rs            # Core kernel library
    arch/             # Architecture-specific code
      x86_64/
        boot.asm      # Boot assembly
        interrupts.rs # IDT and interrupt handlers
    mm/               # Memory management
      paging.rs
      allocator.rs
    vga/              # VGA console driver
      color.rs
      writer.rs
    panic.rs          # Panic handler
  Cargo.toml
  x86_64-comet-os.json  # Cargo target specification
```

## Development

See [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) for design details.

## License

MIT
