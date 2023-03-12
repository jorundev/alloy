# Alloy project structure

The project is separated in two parts:

- Alloy (main crate)
- bootcore

## Main crate

The root `Cargo.toml` defines a [workspace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html) so that every single project crate uses the same `target/` directory.


## bootcore

`bootcore` is the part of the code that is executed before `kernel_main`. It is responsible of:
- Getting information from the bootloader, translating it into a format that the kernel can understand
- Map the kernel to the higher half of the address space
- Map the framebuffer / text buffer (if there is one)
- Jump to `kernel_main` (in the higher half)

This crate is situated in the `bootcore/` directory.

It is built into a static library. All the sections of that library then gets linked into the main kernel ELF in the `.boot.*` sections. This is done so that all of the bootcore's code is in the lower half of the address space. bootcore also has its own panic handler as a result.
