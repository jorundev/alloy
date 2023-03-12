use memory::{PhysAddr, Addr};

use crate::x86::serial;
use core::fmt::Write;

pub unsafe fn boot(multiboot_info_ptr: u32) -> ! {
    let _ = writeln!(serial::out(), "\nalloy kernel");

    let boot_info = unsafe {
        match multiboot2::load(multiboot_info_ptr as usize) {
            Ok(info) => info,
            Err(e) => {
                let _ = writeln!(serial::out(), "Error: {:#?}", e);
                loop {}
            }
        }
    };

    if let Some(elf) = boot_info.elf_sections_tag() {
        for section in elf.sections() {
            let _ = writeln!(
                serial::out(),
                "{:?} {:?} ({:#x} -> {:#x})",
                section.name(),
                section.section_type(),
                section.start_address(),
                section.end_address()
            );
			let start_address = PhysAddr::new(section.start_address() as usize);
			if section.size() >= 0x20 {
				memory::utils::print_memory_ptr(serial::out(), start_address.as_ptr::<u8>(), 0x20);
				let _ = writeln!(serial::out());
			}
        }
    }
    loop {}
}
