use core::panic::PanicInfo;

#[panic_handler]
unsafe fn panic_handler(_info: &PanicInfo) -> ! {
	loop {}
}
