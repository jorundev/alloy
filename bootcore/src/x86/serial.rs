use core::fmt::Write;
use super::ports::outb;

pub struct SerialOut;

static mut SERIALOUT: SerialOut = SerialOut;

pub unsafe fn out() -> &'static mut SerialOut {
    &mut SERIALOUT
}

impl Write for SerialOut {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.bytes() {
            unsafe {
                outb(0x3F8, c);
                if c == '\n' as u8 {
                    outb(0x3F8, '\r' as u8);
                }
            }
        }
        Ok(())
    }
}
