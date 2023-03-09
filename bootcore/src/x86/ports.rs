use core::arch::asm;

#[allow(dead_code)]
#[inline(always)]
pub unsafe fn outb(port: u16, val: u8) {
    asm!("outb %al, %dx", in("al") val, in("dx") port, options(att_syntax));
}

#[allow(dead_code)]
#[inline(always)]
pub unsafe fn inb(port: u16) -> u8 {
    let ret: u8;
    asm!("inb %dx, %al", in("dx") port, out("al") ret, options(att_syntax));
    ret
}

#[allow(dead_code)]
#[inline(always)]
pub unsafe fn outw(port: u16, val: u16) {
    asm!("outw %ax, %dx", in("ax") val, in("dx") port, options(att_syntax));
}

#[allow(dead_code)]
#[inline(always)]
pub unsafe fn inw(port: u16) -> u16 {
    let ret: u16;
    asm!("inw %dx, %ax", in("dx") port, out("ax") ret, options(att_syntax));
    ret
}

#[allow(dead_code)]
#[inline(always)]
pub unsafe fn outl(port: u16, val: u32) {
    asm!("outl %eax, %dx", in("eax") val, in("dx") port, options(att_syntax));
}

#[allow(dead_code)]
#[inline(always)]
pub unsafe fn inl(port: u16) -> u32 {
    let ret: u32;
    asm!("inl %dx, %eax", out("eax") ret, in("dx") port, options(att_syntax));
    ret
}
