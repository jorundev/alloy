KERNEL_ISO = alloy.iso
ISODIR=.tmp.dir.iso

all: $(KERNEL_ISO)

$(ISODIR):
	mkdir -p $(ISODIR)
	mkdir -p $(ISODIR)/boot
	mkdir -p $(ISODIR)/boot/grub

$(KERNEL_ISO): $(KERNEL_ELF) | $(ISODIR)
	cp res/grub/grub.cfg $(ISODIR)/boot/grub/grub.cfg
	cp $(KERNEL_ELF) $(ISODIR)/boot/kernel.elf
	grub-mkrescue -o $(KERNEL_ISO) $(ISODIR)
