FEATURES=$(ARCH)

ifeq ($(ARCH),i386)
	ifeq ($(DISABLE_MULTIBOOT2),)
		FEATURES+=multiboot2
	endif
endif

BOOTCORE_DIR=bootcore

comma:= ,
FEATURES_FLAG=$(subst $() $(),$(comma),$(FEATURES))

BRUSTFLAGS="-C link-arg=-T$(LAYOUT) -C link-arg=-zmax-page-size=0x1000"
BCARGOARGS=--target=../$(TARGET) $(PROFILE_FLAG) --features=$(FEATURES_FLAG)

LIBBOOTCORE_DIR=target/$(TRIPLET)/$(PROFILE)
LIBBOOTCORE=$(LIBBOOTCORE_DIR)/libbootcore.a

# Phony target because cargo takes care of everything
.PHONY: $(LIBBOOTCORE)
$(LIBBOOTCORE):
	cd $(BOOTCORE_DIR) && RUSTFLAGS=$(BRUSTFLAGS) $(CARGO) rustc $(BCARGOARGS)
