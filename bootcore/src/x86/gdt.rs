use gdt::x86::{Descriptor, GlobalDescriptorTable, GDTR};

// AT&T syntax is needed for a long jump (https://www.mail-archive.com/llvm-bugs@lists.llvm.org/msg50847.html)
core::arch::global_asm!(include_str!("gdt.s"), options(att_syntax));

extern "cdecl" {
    fn load_gdt(gdtr: *const GDTR);
}

pub unsafe fn setup(address: u32) {
    let mut gdt = GlobalDescriptorTable::new(address);

    gdt.add_entry(Descriptor::flat_kernel_code_segment()); /* 0x08 */
    gdt.add_entry(Descriptor::flat_kernel_data_segment()); /* 0x10 */
    gdt.add_entry(Descriptor::flat_kernel_stack_segment()); /* 0x18 */

    gdt.add_entry(Descriptor::flat_user_code_segment()); /* 0x20 */
    gdt.add_entry(Descriptor::flat_user_data_segment()); /* 0x28 */
    gdt.add_entry(Descriptor::flat_user_stack_segment()); /* 0x30 */

    let gdtr = gdt.gdtr();

    load_gdt(&gdtr as *const GDTR);
}
