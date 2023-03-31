use core::panic::PanicInfo;

/* bootcore should never panic because the panic handler is not linked properly */
#[panic_handler]
fn bootcore_panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}

/*
#[panic_handler]
#[no_mangle]
pub unsafe fn bootcore_panic_handler(info: &PanicInfo) -> ! {
    let _ = writeln!(serial::out(), "\nKernel Panic!");

    if let Some(location) = info.location() {
        let _ = writeln!(
            serial::out(),
            "at {}:{}:{}",
            location.file(),
            location.line(),
            location.column()
        );
    }

    if let Some(message) = info.message() {
        let _ = writeln!(serial::out(), "message: {:?}", message);
    }

    loop {}
}
*/
