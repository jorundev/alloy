#![no_std]

pub mod x86;
pub mod utils;

pub trait Addr {
    fn as_raw(&self) -> usize;
    fn as_ptr<T>(&self) -> *const T {
        self.as_raw() as *const T
    }

    fn as_mut_ptr<T>(&mut self) -> *mut T {
        self.as_raw() as *mut T
    }

    fn is_null(&self) -> bool {
        self.as_raw() == 0
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct PhysAddr(usize);

impl PhysAddr {
    pub fn new(raw: usize) -> Self {
        Self(raw)
    }
}

impl Addr for PhysAddr {
    fn as_raw(&self) -> usize {
        self.0
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct VirtAddr(usize);

impl VirtAddr {
    pub fn new(raw: usize) -> Self {
        Self(raw)
    }
}

impl Addr for VirtAddr {
    fn as_raw(&self) -> usize {
        self.0
    }
}
