/// Prints a memory region. Much like hexdump
#[allow(dead_code)]
pub fn print_memory_ptr<W: core::fmt::Write, T>(writer: &mut W, ptr: *const T, size: usize) {
    let ptr = ptr as *const u8;
    unsafe {
        let slice = core::slice::from_raw_parts(ptr, size);
        for (i, lines) in slice.chunks(8).enumerate() {
            write!(writer, "\n{:08.8x}: ", ptr.offset(8 * i as isize) as usize).unwrap();
            for (i, byte) in lines.iter().enumerate() {
                write!(writer, "{:02.2x}", byte).unwrap();
                if i + 1 != lines.len() {
                    write!(writer, " ").unwrap();
                }
            }
        }
    }
}

/// Prints a memory region. Much like hexdump
#[allow(dead_code)]
pub fn print_memory_ref<W: core::fmt::Write, T>(writer: &mut W, ptr: &T, size: usize) {
    print_memory_ptr(writer, ptr as *const T, size);
}

/// Prints a memory region. Much like hexdump
#[allow(dead_code)]
pub fn print_memory<W: core::fmt::Write>(writer: &mut W, ptr: usize, size: usize) {
    print_memory_ptr(writer, ptr as *const (), size);
}
