BOOTLOADER?=GRUB

include mk/config.mk
ifeq ($(BOOTLOADER),GRUB)
include mk/grub.mk
else ifeq ($(BOOTLOADER),limine)
include mk/limine.mk
else
$(error Unknown iso maker: "$(BOOTLOADER)")
endif
include mk/qemu.mk
include mk/bootcore.mk

RUSTFLAGS="-C link-arg=-T$(LAYOUT) -C link-arg=-zmax-page-size=0x1000 --verbose"
CARGOARGS=--target=./$(TARGET) $(PROFILE_FLAG)

.PHONY: elf
elf: $(KERNEL_ELF)

.PHONY: $(KERNEL_ELF)
$(KERNEL_ELF):
	RUSTFLAGS=$(RUSTFLAGS) LIBBOOTCORE_DIR=$(LIBBOOTCORE_DIR) $(CARGO) rustc $(CARGOARGS)

clean:
	$(RM) -r $(KERNEL_ISO) isodir
