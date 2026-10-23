#!/bin/bash
set -e

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BUILD_DIR="$REPO_ROOT/build"
ISO_PATH="$BUILD_DIR/kaustavos.iso"
PERSIST_IMG="$BUILD_DIR/usb_persistence.img"

if [ ! -f "$ISO_PATH" ]; then
    echo "ISO not found. Building KaustavOS first..."
    "$REPO_ROOT/scripts/build_iso.sh"
fi

# Ensure persistent test drive exists
if [ ! -f "$PERSIST_IMG" ]; then
    echo "=== Creating Virtual USB Persistent Drive (64MB ext4) ==="
    dd if=/dev/zero of="$PERSIST_IMG" bs=1M count=64 status=none
    mkfs.ext4 -F -L KAUSTAV_DATA "$PERSIST_IMG" >/dev/null 2>&1
    echo "Persistent test image created: $PERSIST_IMG (Label: KAUSTAV_DATA)"
fi

# Check if GUI display is available (DISPLAY or WAYLAND_DISPLAY)
QEMU_DISPLAY_ARGS=""
if [ -z "$DISPLAY" ] && [ -z "$WAYLAND_DISPLAY" ]; then
    echo "No X11/Wayland display detected. Starting QEMU in nographic serial mode..."
    QEMU_DISPLAY_ARGS="-nographic -serial mon:stdio"
else
    echo "Display server detected. Starting QEMU with graphical window and serial console..."
    QEMU_DISPLAY_ARGS="-serial stdio"
fi

echo "=========================================================="
echo "               Booting KaustavOS in QEMU                  "
echo "=========================================================="
echo " Booting from : $ISO_PATH"
echo " Persistence  : $PERSIST_IMG"
echo " Memory       : 1024 MB"
echo " Tip: Press Ctrl-A, X in terminal to terminate QEMU."
echo "=========================================================="

qemu-system-x86_64 \
    -cdrom "$ISO_PATH" \
    -drive file="$PERSIST_IMG",format=raw \
    -m 1024M \
    $QEMU_DISPLAY_ARGS
