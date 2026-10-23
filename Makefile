.PHONY: all iso userland run run-cli test clean help

all: iso

help:
	@echo "KaustavOS Build System"
	@echo "Available targets:"
	@echo "  make iso      : Build the complete bootable hybrid ISO (Limine + Kernel + Rust UI)"
	@echo "  make userland : Compile the Rust userspace UI and shell in release mode"
	@echo "  make run      : Boot KaustavOS in QEMU with persistence enabled"
	@echo "  make run-cli  : Boot KaustavOS in QEMU (headless serial mode)"
	@echo "  make test     : Run automated interactive verification in QEMU"
	@echo "  make clean    : Remove build artifacts"

userland:
	cd userland && cargo build --release

iso:
	bash scripts/build_iso.sh

run:
	bash scripts/run_qemu.sh

run-cli:
	qemu-system-x86_64 -cdrom build/kaustavos.iso \
		-drive file=build/usb_persistence.img,format=raw \
		-m 1024M -nographic -serial mon:stdio

test:
	bash scripts/test_interactive.sh

clean:
	rm -rf build/initramfs build/iso_root build/*.cpio.gz
	cd userland && cargo clean || true
