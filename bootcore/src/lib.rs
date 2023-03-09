#![no_std]

use core::panic::PanicInfo;

#[cfg(feature = "x86")]
mod x86;

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
