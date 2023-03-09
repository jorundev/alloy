ENTRY(_start)
OUTPUT_FORMAT(elf32-i386)

SECTIONS {
	. = 1M;

	.boot.multiboot : {
		*(.boot.multiboot)
	}
	.boot.text : {
		*(.boot.text)
	}
	.boot.data : {
		*(.boot.data)
	}
	.boot.rodata : {
		*(.boot.rodata)
	}
	.boot.bss : {
		*(.boot.bss)
	}

	.text : {
		KEEP(*(.text))
		KEEP(*(.text.*))
	}

	.data : {
		KEEP(*(.data))
		KEEP(*(.data.*))
	}

	.rodata : {
		KEEP(*(.rodata))
		KEEP(*(.rodata.*))
	}

	/DISCARD/ : {
        *(.comment)
        *(.eh_frame)
        *(.note.gnu.build-id)
    }
}
