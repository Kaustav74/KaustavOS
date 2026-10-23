#!/bin/bash
set -e

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BUILD_DIR="$REPO_ROOT/build"
ISO_ROOT="$BUILD_DIR/iso_root"
LIMINE_DIR="$REPO_ROOT/bootloader/limine"

echo "=========================================================="
echo "          Building KaustavOS Bootable ISO & IMG           "
echo "=========================================================="

# 1. Build Userspace and Initramfs
"$REPO_ROOT/scripts/build_rootfs.sh"

# 2. Prepare ISO Root Directory Structure
echo "=== Preparing ISO Layout ==="
rm -rf "$ISO_ROOT"
mkdir -p "$ISO_ROOT"/boot
mkdir -p "$ISO_ROOT"/EFI/BOOT

# Find host linux kernel
KERNEL_SRC=$(ls -1 /boot/vmlinuz* 2>/dev/null | head -n 1)
if [ -z "$KERNEL_SRC" ]; then
    echo "ERROR: Linux kernel vmlinuz not found in /boot!"
    exit 1
fi
echo "Using Kernel: $KERNEL_SRC"
cp "$KERNEL_SRC" "$ISO_ROOT/boot/vmlinuz"

# Copy Initramfs
cp "$BUILD_DIR/initramfs.cpio.gz" "$ISO_ROOT/boot/initramfs.cpio.gz"

# Copy Limine Bootloader Components
echo "=== Installing Limine Bootloader Files ==="
cp "$LIMINE_DIR/limine-bios-cd.bin" "$ISO_ROOT/"
cp "$LIMINE_DIR/limine-uefi-cd.bin" "$ISO_ROOT/"
cp "$LIMINE_DIR/limine-bios.sys" "$ISO_ROOT/"
cp "$REPO_ROOT/bootloader/limine.conf" "$ISO_ROOT/"
cp "$REPO_ROOT/bootloader/limine.conf" "$ISO_ROOT/boot/"

# EFI bootloaders for Rufus ISO extraction mode
cp "$LIMINE_DIR/BOOTX64.EFI" "$ISO_ROOT/EFI/BOOT/BOOTX64.EFI"
cp "$LIMINE_DIR/BOOTIA32.EFI" "$ISO_ROOT/EFI/BOOT/BOOTIA32.EFI"

# 3. Create Hybrid ISO with xorriso
echo "=== Generating Hybrid ISO with xorriso ==="
ISO_OUT="$BUILD_DIR/kaustavos.iso"
rm -f "$ISO_OUT"

xorriso -as mkisofs \
    -b limine-bios-cd.bin \
    -no-emul-boot \
    -boot-load-size 4 \
    -boot-info-table \
    --efi-boot limine-uefi-cd.bin \
    -efi-boot-part \
    --efi-boot-image \
    --protective-msdos-label \
    "$ISO_ROOT" \
    -o "$ISO_OUT"

# 4. Install Limine BIOS Boot Stages for USB / HDD Boot
echo "=== Installing Limine Hybrid Stages ==="
"$LIMINE_DIR/limine" bios-install "$ISO_OUT"

# 5. Create raw IMG mirror for Rufus raw disk writing
IMG_OUT="$BUILD_DIR/kaustavos.img"
cp -f "$ISO_OUT" "$IMG_OUT"

echo ""
echo "=========================================================="
echo "          KaustavOS Build Successfully Completed!         "
echo "=========================================================="
echo " Output files:"
echo "   ISO Image : $ISO_OUT"
echo "   Raw Image : $IMG_OUT"
echo " Size: $(du -h "$ISO_OUT" | cut -f1)"
echo ""
echo " Ready for Rufus USB writing (UEFI & BIOS) and QEMU boot."
echo "=========================================================="
