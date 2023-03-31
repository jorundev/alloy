.section .text, "a"
.type load_gdt, @function
load_gdt:
	cli
	movl 4(%esp), %edi
	lgdt (%edi)
	jmp $0x08, $flush

flush:
	movw $0x10, %ax
	mov %ax, %ds
	mov %ax, %es
	mov %ax, %fs
	mov %ax, %gs
	movw $0x18, %ax
    mov %ax, %ss
	ret
