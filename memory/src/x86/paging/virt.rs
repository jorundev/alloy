use crate::{Addr, VirtAddr};

pub trait X86VirtualAddress : Addr {
	fn p4k_page(&self) -> u32 {
		(self.as_raw() & !0xFFF) as u32
	}

	fn p4k_page_directory_index(&self) -> u32 {
		(self.as_raw() >> 22) as u32
	}

	fn p4k_page_table_index(&self) -> u32 {
		((self.as_raw() >> 12) & 0x3FF) as u32
	}
}

impl X86VirtualAddress for VirtAddr {}
