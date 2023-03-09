ARCH?=i386
CARGO?=cargo
PROFILE?=debug

ifeq ($(ARCH),i386)
	LAYOUT=layouts/x86.ld
	TRIPLET=i386-unknown-none
	TARGET=triplets/i386-unknown-none.json
else ifeq ($(ARCH),)
$(error No ARCH provided)
else
$(error Unsupported ARCH: "$(ARCH)")
endif

ifeq ($(PROFILE),release)
	PROFILE_FLAG=--release
else ifneq ($(PROFILE),debug)
$(error Unsupported PROFILE: "$(PROFILE)")
endif

KERNEL_ELF=target/$(TRIPLET)/$(PROFILE)/alloy.elf
