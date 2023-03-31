QEMU=qemu-system-$(QEMU_ARCH)
QEMUFLAGS=-serial stdio

ifeq ($(ARCH),i386)
	efi=no
	QEMU_ARCH=i386
	QEMU_MACHINE?=pc
	QEMUFLAGS+=-smp 4 -m 200
else
$(error Unsupported ARCH for QEMU "$(ARCH)")
endif

.PHONY: qemu
qemu: $(KERNEL_ISO)
	$(QEMU) $(QEMUFLAGS) -cdrom $(KERNEL_ISO)
