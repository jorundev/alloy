#![no_std]
#![allow(dead_code)]

pub mod utils;
pub mod x86;

pub trait MemoryArea {
    fn start_address(&self) -> usize;
    fn end_address(&self) -> usize;

    fn len(&self) -> usize {
        self.end_address() - self.start_address()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Frame<const PAGE_SIZE: usize> {
    pub number: usize,
}

impl<const PAGE_SIZE: usize> Frame<PAGE_SIZE> {
    pub fn from_addr(address: PhysAddr) -> Self {
        Frame {
            number: address.as_raw() / PAGE_SIZE,
        }
    }

    pub fn address(&self) -> PhysAddr {
        PhysAddr(self.number * PAGE_SIZE)
    }

    pub fn as_ptr<T>(&self) -> *const T {
        self.address().0 as *const T
    }

    pub fn as_mut_ptr<T>(&self) -> *mut T {
        self.address().0 as *mut T
    }
}

pub trait FrameAllocator<const PAGE_SIZE: usize> {
    fn allocate_frame(&mut self) -> Option<Frame<PAGE_SIZE>>;
    fn deallocate_frame(&mut self, frame: Frame<PAGE_SIZE>);
}

pub type Frame4K = Frame<0x1000>;

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
pub struct PhysAddr(pub usize);

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
pub struct VirtAddr(pub usize);

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
