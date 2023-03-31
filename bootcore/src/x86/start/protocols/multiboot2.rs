use crate::x86::{gdt, serial};
use core::fmt::Write;
use memory::x86::paging::phys::BannedFrames;
use memory::{x86::paging::phys::AreaFrameAllocator, Frame4K};
use memory::{FrameAllocator, PhysAddr};
use multiboot2::{ElfSectionType, FramebufferType, MemoryAreaType};

use self::area::MemoryArea;

mod area;

pub unsafe fn boot(multiboot_info_ptr: u32) -> ! {
    let _ = writeln!(serial::out(), "\n[BOOT] Alloy kernel");
    let gdt_location = 0x800;

    let boot_info = unsafe {
        match multiboot2::load(multiboot_info_ptr as usize) {
            Ok(info) => info,
            Err(e) => {
                let _ = writeln!(serial::out(), "Error: {:#?}", e);
                loop {}
            }
        }
    };

    let mut frame_allocator = if let Some(map) = boot_info.memory_map_tag() {
        if let Some(elf) = boot_info.elf_sections_tag() {
            let available_iter = map
                .memory_areas()
                .filter(|area| area.typ() == MemoryAreaType::Available)
                .map(|area| MemoryArea::new(area));

            let banned_elf_sections = elf
                .sections()
                .filter(|section| section.section_type() != ElfSectionType::Note)
				.filter(|section| section.start_address() < 0xC0000000) /* Addresses above 0xC000000 are virtual addresses and should not be skipped */
                .map(|section| {
                    BannedFrames::new(
                        Frame4K::from_addr(PhysAddr(section.start_address() as usize)),
                        Frame4K::from_addr(PhysAddr(section.end_address() as usize)),
                    )
                });

            /*for s in banned_elf_sections {
                let _ = writeln!(serial::out(), "{:?}", s);
            }
            loop {}*/

            AreaFrameAllocator::new(available_iter, Some(banned_elf_sections))
        } else {
            let _ = writeln!(serial::out(), "Error: No ELF info provided by bootloader");
            loop {}
        }
    } else {
        let _ = writeln!(serial::out(), "Error: No memory map provided by bootloader");
        loop {}
    };

    gdt::setup(gdt_location);

    let _ = writeln!(
        serial::out(),
        "[BOOT] Global Descriptor Table loaded at address {:#x}",
        gdt_location,
    );

    if let Some(framebuffer) = boot_info.framebuffer_tag() {
        if let Ok(framebuffer) = framebuffer {
            match framebuffer.buffer_type {
                FramebufferType::Indexed { palette: _ } => {
                    let _ = writeln!(serial::out(), "[BOOT] Indexed color framebuffer found at address {:#x}. This type of framebuffer is not supported", framebuffer.address);
                }
                FramebufferType::RGB {
                    red: _,
                    green: _,
                    blue: _,
                } => {
                    let _ = writeln!(
                        serial::out(),
                        "[BOOT] RGB framebuffer found at address {:#x}",
                        framebuffer.address
                    );
                }
                FramebufferType::Text => {
                    let _ = writeln!(
                        serial::out(),
                        "[BOOT] VGA text buffer found at address {:#x}",
                        framebuffer.address
                    );
                }
            }
        }
    } else {
        let _ = writeln!(serial::out(), "[BOOT] No framebuffer found");
    }

    loop {
        let frame = frame_allocator.allocate_frame();

        if let Some(frame) = frame {
            let _ = writeln!(serial::out(), "{:#x}", frame.address().0);
        } else {
            let _ = writeln!(serial::out(), "None");
            loop {}
        }
    }

    loop {}
}
