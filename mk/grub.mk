KERNEL_ISO = alloy.iso

all: $(KERNEL_ISO)

isodir:
	mkdir -p isodir
	mkdir -p isodir/boot
	mkdir -p isodir/boot/grub

$(KERNEL_ISO): $(KERNEL_ELF) | isodir
	cp res/grub/grub.cfg isodir/boot/grub/grub.cfg
	cp $(KERNEL_ELF) isodir/boot/kernel.elf
	grub-mkrescue -o $(KERNEL_ISO) isodir
	$(RM) -r isodir
