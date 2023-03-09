use core::fmt::Write;
use crate::x86::serial;

pub unsafe fn boot(_boot_info_ptr: u32) -> ! {
	let _ = writeln!(serial::out(), "Hello, world!");
    loop {}
}
