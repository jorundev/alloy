.section .boot.multiboot, "a", @note

.set	MB2_MAGIC,		0xE85250D6
.set	MB2_ARCH_X86,	0

# Multiboot 2 header
.align 8
mb2start:
	.long	MB2_MAGIC
	.long	MB2_ARCH_X86
	.long	mb2end - mb2start
	.long	0x100000000 - (MB2_MAGIC + MB2_ARCH_X86 + (mb2end - mb2start))

	# .word	5 # Framebuffer
	# .word	0 # Flags
	# .long	16 # Size
	# .long	1280 # Width
	# .long	720 # Height

	.word	0
	.word	0
	.long	8
mb2end:

.section .text
.global _start
.type _start, @function
_start:
	lea	esp, boot_stack_top
	push ebx
	push eax
	call boot
halt:
	hlt
	jmp halt

.section .bss, "aw", @nobits
boot_stack_bottom:
	.skip 4096
boot_stack_top:
