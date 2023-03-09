use core::arch::global_asm;

mod protocols;

global_asm!(include_str!("boot.s"));

#[no_mangle]
pub unsafe extern "C" fn boot(eax: u32, ebx: u32) -> ! {
    cfg_if::cfg_if! {
        if #[cfg(feature = "multiboot2")] {
            const MULTIBOOT2_MAGIC: u32 = 0x36D76289;

            if eax == MULTIBOOT2_MAGIC {
                protocols::multiboot2::boot(ebx);
            }
        }
    }
    panic!("Unrecognized boot protocol");
}
