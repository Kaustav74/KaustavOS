# KaustavOS Simple Start Guide

Get KaustavOS built, tested in QEMU, and written to a USB drive in minutes.

---

## 1. Quick Build (One Command)

In your terminal (WSL or Linux with `cargo`, `xorriso`, and `make` installed):

```bash
make iso
```

This single command:
1. Compiles the **KaustavOS Rust Core Engine** (`userland/`) in release mode.
2. Packages the lightweight initial ramdisk (`build/initramfs.cpio.gz`).
3. Embeds the **Limine v8.x** bootloader (both UEFI and BIOS stages).
4. Generates the bootable hybrid image:
   - `build/kaustavos.iso` (for Rufus ISO or DD mode)
   - `build/kaustavos.img` (raw disk image)

---

## 2. Test in QEMU (Windows or Linux)

### On Windows:
Double-click:
```cmd
run_qemu.bat
```
*(Automatically detects your native QEMU or launches seamlessly via WSL!)*

### On Linux / WSL:
```bash
make run
```
or
```bash
bash scripts/run_qemu.sh
```

---

## 3. Burn to USB with Rufus (Real Hardware)

1. Open **Rufus** on Windows.
2. Under **Device**, choose your USB drive (e.g. 64 GB USB stick).
3. Under **Boot selection**, select `build\kaustavos.iso`.
4. Under **Persistent partition size**, drag the slider to your desired size (e.g., **8 GB** to **32 GB**).
5. Click **START** -> Select **Write in ISO Image mode (Recommended)**.
6. Insert the USB into any PC and boot from USB via your motherboard's boot menu key (`F12`, `F9`, or `F8`)!

---

## 4. First Boot Experience

1. **Branding Splash**: Large stylized `KaustavOS` banner with hardware diagnostics.
2. **Personal Threads**: Animated sequence introducing:
   - `School Journeys`
   - `CareGuardian`
   - `AI Systems & Robotics Lab`
   - `Decentralized Resilience Mesh`
3. **OS Style Chooser**: Press `[1]` for Windows style, `[2]` for macOS style, or `[3]` for Linux style.
4. **Themed Shell**: Try typing:
   - `note My first USB note` -> Saves immediately to USB persistent partition.
   - `notes` -> Displays saved notes.
   - `theme 2` -> Hot-swaps to macOS style on the fly!
   - `threads` -> Explores personal threads in depth.
   - `sys` -> Checks memory and persistence status.
