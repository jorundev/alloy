#![no_main]
#![no_std]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Export _start symbol from libbootcore
extern "C" {
    pub fn _start() -> !;
}
