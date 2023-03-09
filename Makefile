ISO_MAKER?=GRUB

include mk/config.mk
ifeq ($(ISO_MAKER),GRUB)
include mk/grub.mk
else ifeq ($(ISO_MAKER),limine)
else
$(error Unknown iso maker: "$(ISO_MAKER)")
endif
include mk/qemu.mk
include mk/bootcore.mk

RUSTFLAGS="-C link-arg=-T$(LAYOUT) -C link-arg=-zmax-page-size=0x1000"
CARGOARGS=--target=./$(TARGET) $(PROFILE_FLAG)

$(KERNEL_ELF): $(LIBBOOTCORE)
	RUSTFLAGS=$(RUSTFLAGS) LIBBOOTCORE_DIR=$(LIBBOOTCORE_DIR) $(CARGO) rustc $(CARGOARGS)

clean:
	$(RM) -r $(KERNEL_ISO) isodir
