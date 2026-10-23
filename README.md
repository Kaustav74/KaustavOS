# KaustavOS

**A practical, bootable, lightweight operating system image built with safe Rust and the Limine bootloader.**

## Current Status (Updated)
- Prebuilt userspace binary available: `prebuilt/kaustavos-ui` (release build of the Rust UI).
- Full ISO generation requires a Linux environment (WSL recommended on Windows) with `cargo`, `xorriso`, `cpio`, `busybox`, and a host Linux kernel (`/boot/vmlinuz*`).
- The live system is a Linux initramfs + Rust UI (not a pure from-scratch kernel). The bare-metal skeleton remains in `kernel/`.

## Quickstart (on Linux / WSL)

```bash
# Install dependencies (example for Ubuntu/Debian)
sudo apt update && sudo apt install -y cargo xorriso cpio busybox make

# Build the ISO
make iso

# This produces:
# build/kaustavos.iso   ← Rufus-ready hybrid ISO
# build/kaustavos.img
```

Then flash `build/kaustavos.iso` with Rufus (see `docs/BOOT_USB.md`).

## What You Get
- Custom KaustavOS splash
- Sequential personal threads display
- Windows / macOS / Linux style chooser
- Themed terminal shell with notes, threads, theme switching
- USB persistence (when a labeled partition is present)

## Repository
https://github.com/Kaustav74/KaustavOS

Created by Kaustav Nath.
