# Booting KaustavOS from USB

## Requirements
- 64 GB USB 2.0 pendrive (or larger).
- A host machine to build and write the image.

## Planned Method
1. Build the kernel with cargo (target x86_64-unknown-none or similar).
2. Use Limine or GRUB as the bootloader to create a bootable ISO/USB image.
3. Partition the USB with a small EFI/BIOS boot partition and a large data partition for persistence.
4. Write the image using tools like `dd` or Rufus.

## Testing
- First test in QEMU: `qemu-system-x86_64 -cdrom kaustavos.iso` or with USB passthrough.
- Once stable, write to real USB and boot on hardware.

Full instructions will be added as the build system matures.
