ENTRY(_start)
OUTPUT_FORMAT(elf32-i386)

KERNEL_VBASE = 0xC0000000;

SECTIONS {
	.boot.multiboot : {
		*(.boot.multiboot)
	}

	. = 1M;

	.boot.text : {
		*libbootcore*.a:*(.text .text.*);
		. = ALIGN(0x1000);
	}

	. = ALIGN(0x1000);
	.boot.rodata : {
		*libbootcore*.a:*(.rodata .rodata.*);
		. = ALIGN(0x1000);
	}

	. = ALIGN(0x1000);
	.boot.data : {
		*libbootcore*.a:*(.data .data.*);
		. = ALIGN(0x1000);
	}

	. = ALIGN(0x1000);
	.boot.bss : {
		*libbootcore*.a:*(.bss .bss.*);
		. = ALIGN(0x1000);
	}

	. += KERNEL_VBASE;
	.text : AT(ADDR(.text) - KERNEL_VBASE) {
		*(.text .text.*)
		. = ALIGN(0x1000);
	}

	.rodata : AT(ADDR(.rodata) - KERNEL_VBASE) {
		*(.rodata .rodata.*)
		. = ALIGN(0x1000);
	}

	.data : AT(ADDR(.data) - KERNEL_VBASE) {
		*(.data .data.*)
		. = ALIGN(0x1000);
	}

	.bss : AT(ADDR(.bss) - KERNEL_VBASE) {
		*(.bss .bss.*)
		. = ALIGN(0x1000);
	}

	/DISCARD/ : {
        *(.comment)
        *(.eh_frame)
        *(.note.gnu.build-id)
		*(.debug*)
    }
}
