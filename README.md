# KaustavOS

Lightweight educational kernel project focused on security principles (minimal TCB, isolation, Rust memory safety).

## Simple Goal
A bootable starting point that can grow into a USB-bootable system with:
- KaustavOS splash screen
- Brief display of personal threads (School Journeys, CareGuardian, etc.)
- Choice of desktop style: Windows-like / macOS-like / Linux-like
- Persistent storage on the USB drive

**This is not a full OS yet.** It is a clean, simple skeleton you can extend. Full Windows/macOS/Linux compatibility interfaces require years of work and cannot be created by copying proprietary software (that would be illegal).

## How to Start Simply
1. Install Rust (nightly recommended for bare-metal).
2. Install QEMU for testing.
3. Use an open-source bootloader like **Limine** (see links below).
4. Build the kernel and boot it in QEMU first.
5. Once stable, create a USB image.

### Recommended Learning Path (Open Source)
- Philipp Oppermann's "Writing an OS in Rust" series: https://os.phil-opp.com/
- Limine Rust templates (public GitHub repos)
- OSDev Wiki

## Current Files
- `kernel/` — Rust no_std skeleton with entry point and comments for the boot sequence.
- `docs/` — Architecture and USB boot notes.

## Next Easy Steps
- Add serial or VGA text output so it prints "KaustavOS".
- Implement a simple menu for the OS mode chooser.
- Integrate Limine for real bootable images.

Created by Kaustav Nath.
