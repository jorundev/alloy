pub struct GlobalDescriptorTable {
    address: u32,
    last_entry_index: u32,
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed(2))]
pub struct GDTR {
    size: u16,
    offset: u32,
}

impl GlobalDescriptorTable {
    pub unsafe fn new(address: u32) -> Self {
        let ret = Self {
            address,
            last_entry_index: 1, // 0 is the NULL descriptor
        };

        ret.write_entry(0, Descriptor::null());
        ret
    }

    pub unsafe fn add_entry(&mut self, descriptor: Descriptor) {
        self.write_entry(self.last_entry_index, descriptor);
        self.last_entry_index += 1;
    }

    pub fn gdtr(&self) -> GDTR {
        GDTR {
            size: (self.last_entry_index * 8 - 1) as u16,
            offset: self.address,
        }
    }

    unsafe fn write_entry(&self, index: u32, descriptor: Descriptor) {
        let limit_bits =
            (descriptor.limit as u64 & 0xFFFF) | ((descriptor.limit as u64 & 0xF0000) << 32);

        let base_bits = ((descriptor.base as u64 & 0xFFFFFF) << 16)
            | ((descriptor.base as u64 & 0xFF000000) << 32);

        let raw = limit_bits
            | base_bits
            | ((descriptor.access_byte as u64) << 40)
            | ((descriptor.flags as u64) << 52);

        core::ptr::write_volatile((self.address + 8 * index) as *mut u64, raw);
    }
}

#[derive(Debug, PartialEq)]
pub struct Descriptor {
    base: u32,
    limit: u32,
    access_byte: u8,
    flags: u8,
}

impl Descriptor {
    pub const fn new(base: u32, limit: u32, access_byte: u8, flags: u8) -> Self {
        Descriptor {
            base,
            limit,
            access_byte,
            flags,
        }
    }

    pub const fn null() -> Self {
        Descriptor {
            base: 0,
            limit: 0,
            access_byte: 0,
            flags: 0,
        }
    }

    pub const fn flat_kernel_code_segment() -> Self {
        Descriptor {
            base: 0x0,
            limit: 0xFFFFF,
            access_byte: 0b10011000,
            flags: 0b1100,
        }
    }

    pub const fn flat_kernel_data_segment() -> Self {
        Descriptor {
            base: 0x0,
            limit: 0xFFFFF,
            access_byte: 0b10010010,
            flags: 0b1100,
        }
    }

    pub const fn flat_kernel_stack_segment() -> Self {
        Descriptor {
            base: 0x0,
            limit: 0x0,
            access_byte: 0b10010110,
            flags: 0b1100,
        }
    }

    pub const fn flat_user_code_segment() -> Self {
        Descriptor {
            base: 0x0,
            limit: 0xFFFFF,
            access_byte: 0b11111000,
            flags: 0b1100,
        }
    }

    pub const fn flat_user_data_segment() -> Self {
        Descriptor {
            base: 0x0,
            limit: 0xFFFFF,
            access_byte: 0b11110010,
            flags: 0b1100,
        }
    }

    pub const fn flat_user_stack_segment() -> Self {
        Descriptor {
            base: 0x0,
            limit: 0x0,
            access_byte: 0b11110110,
            flags: 0b1100,
        }
    }
}
