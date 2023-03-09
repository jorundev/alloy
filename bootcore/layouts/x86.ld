SECTIONS {
	.boot.multiboot : {
		*(.boot.multiboot)
	}

	.boot.text: {
		*(.text)
		*(.text.*)
	}

	.boot.data: {
		*(.data)
		*(.data.*)
	}

	.boot.rodata: {
		*(.rodata)
		*(.rodata.*)
	}

	.boot.bss: {
		*(.bss)
		*(.bss.*)
	}

	/DISCARD/ : {
        *(.comment)
        *(.eh_frame)
        *(.note.gnu.build-id)
    }
}
