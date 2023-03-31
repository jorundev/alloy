#![no_main]
#![no_std]

use core::panic::PanicInfo;

#[panic_handler]
#[no_mangle]
unsafe fn alloy_panic(_info: &PanicInfo) -> ! {
    core::ptr::write_volatile(0xb8000 as *mut u8, 'A' as u8);
    loop {}
}

// Export _start symbol from libbootcore
extern "C" {
    pub fn _start() -> !;
}
