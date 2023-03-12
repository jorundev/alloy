# Alloy

Alloy is a simple kernel currently targeting the Intel i386 computer. It is written in Rust for safety and practicality reasons (no need to build a cross toolchain; the Rust toolchain should handle everything).

It still is a toy project before all.

## Dependencies

Alloy aims to have as less dependencies as possible, or to be more precise as less dependencies that is not automatically fetched by cargo as possible.

To build Alloy, you will need:
- A Unix(-like) system
- The Rust toolchain (rustup, cargo...)
- Make (GNU Make or BSD Make)

This is enough to make a fully built Alloy ELF Executable compatible with the multiboot2 protocol.

To build a fully bootable ISO, you will need:

- Either GRUB2 (the grub-mkrescue executable) OR Limine
- GNU xorriso

## Build instructions

To get a simple ISO file with GRUB simply do `make` for the debug version

To make a (faster and smaller) release build. Do

`PROFILE=release make`

This will create an `alloy.iso` image in the root.

If you wish to use limine as a bootloader, add `BOOTLOADER=limine` before the make invocation

If you only wish to build the kernel ELF file, use the `elf`makefile rule. This will build the kernel in `target/i386-unknown-none/{debug or release}/alloy.elf`

## Emulating

Do `make qemu` (or `PROFILE=release make qemu` in release mode) to start qemu. You will need qemu-system.

## Known issues

Because of [what seems to be a cargo bug](https://github.com/rust-lang/cargo/issues/6337), the `core` crate is built twice when rebuilding `bootcore`, this adds seconds to the build time. This makes working on the `bootcore` module very slow and inconvenient. I sadly do not know how to fix this as of now.
