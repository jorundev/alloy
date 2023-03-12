KERNEL_ISO = alloy.iso
LIMINE_DEPLOY?=limine-deploy

ifeq ($(LIMINE_SYS),)
	LIMINE_SYS=/usr/share/limine/limine.sys
	ifeq ("$(wildcard $(LIMINE_SYS))","")
		LIMINE_SYS=/usr/local/share/limine/limine.sys
		ifeq ("$(wildcard $(LIMINE_SYS))","")
			$(error Could not find limine.sys. Please provide the path to this file with LIMINE_SYS)
		endif
	endif
endif

ifeq ($(LIMINE_CD),)
	LIMINE_CD=/usr/share/limine/limine-cd.bin
	ifeq ("$(wildcard $(LIMINE_CD))","")
		LIMINE_CD=/usr/local/share/limine/limine-cd.bin
		ifeq ("$(wildcard $(LIMINE_CD))","")
			$(error Could not find limine-cd.bin. Please provide the path to this file with LIMINE_CD)
		endif
	endif
endif

ifeq ($(shell which $(LIMINE_DEPLOY)),)
$(error Could not find limine-deploy. Please provide the path to this file with LIMINE_DEPLOY)
endif

ISODIR=.tmp.dir.iso

all: $(KERNEL_ISO)

$(ISODIR):
	mkdir -p $(ISODIR) $(ISODIR)/boot

$(KERNEL_ISO): $(KERNEL_ELF) | $(ISODIR)
	cp res/limine/limine.cfg $(ISODIR)/boot/limine.cfg
	cp $(LIMINE_SYS) $(ISODIR)/boot/limine.sys
	cp $(LIMINE_CD) $(ISODIR)/boot/limine-cd.bin
	cp $(KERNEL_ELF) $(ISODIR)/boot/kernel.elf
	xorriso -as mkisofs -b /boot/limine-cd.bin \
		-no-emul-boot -boot-load-size 3 -boot-info-table \
		-o $(KERNEL_ISO) $(ISODIR)
	$(LIMINE_DEPLOY) $(KERNEL_ISO)
