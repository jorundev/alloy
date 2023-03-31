#[repr(transparent)]
#[derive(Debug)]
pub struct MemoryArea<'a>(&'a multiboot2::MemoryArea);

impl<'a> MemoryArea<'a> {
    pub fn new(area: &'a multiboot2::MemoryArea) -> Self {
        Self(area)
    }
}

impl<'a> memory::MemoryArea for MemoryArea<'a> {
    #[inline(always)]
    fn start_address(&self) -> usize {
        self.0.start_address() as usize
    }

    #[inline(always)]
    fn end_address(&self) -> usize {
        self.0.end_address() as usize
    }
}
