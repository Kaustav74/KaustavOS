#!/bin/bash
set -e

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BUILD_DIR="$REPO_ROOT/build"
INITRAMFS_DIR="$BUILD_DIR/initramfs"

echo "=== Building KaustavOS Userspace (Rust) ==="
cd "$REPO_ROOT"

# Try compiling if cargo is available, but allow falling back to prebuilt binary
if command -v cargo >/dev/null 2>&1; then
    echo "Running cargo build --release -p kaustavos-ui..."
    cargo build --release -p kaustavos-ui || true
fi

# Find compiled binary (checks workspace root target, subfolder target, or prebuilt fallback)
UI_BIN=""
if [ -f "$REPO_ROOT/target/release/kaustavos-ui" ]; then
    UI_BIN="$REPO_ROOT/target/release/kaustavos-ui"
elif [ -f "$REPO_ROOT/userland/target/release/kaustavos-ui" ]; then
    UI_BIN="$REPO_ROOT/userland/target/release/kaustavos-ui"
elif [ -f "$REPO_ROOT/prebuilt/kaustavos-ui" ]; then
    UI_BIN="$REPO_ROOT/prebuilt/kaustavos-ui"
else
    echo "ERROR: Could not locate compiled kaustavos-ui binary!"
    echo "Please build with 'cargo build --release' or ensure prebuilt/kaustavos-ui exists."
    exit 1
fi
echo "Using UI binary: $UI_BIN"

echo "=== Creating Initramfs Root Directory ==="
rm -rf "$INITRAMFS_DIR"
mkdir -p "$INITRAMFS_DIR"/{bin,sbin,usr/bin,usr/sbin,lib,lib64,proc,sys,dev,data,mnt/usb,tmp,etc}

# Install BusyBox
echo "=== Installing BusyBox ==="
BUSYBOX_BIN=$(which busybox 2>/dev/null || ls /bin/busybox /usr/bin/busybox 2>/dev/null | head -n 1)
if [ -z "$BUSYBOX_BIN" ] || [ ! -x "$BUSYBOX_BIN" ]; then
    echo "ERROR: busybox executable not found on system!"
    echo "Please install busybox (e.g. 'sudo apt install -y busybox' or 'busybox-static')."
    exit 1
fi
echo "Using BusyBox: $BUSYBOX_BIN"
cp "$BUSYBOX_BIN" "$INITRAMFS_DIR/bin/busybox"
chmod 755 "$INITRAMFS_DIR/bin/busybox"

# Create BusyBox Symlinks
cd "$INITRAMFS_DIR/bin"
for cmd in sh ls cat echo mount umount mkdir rm cp mv sync reboot poweroff sleep clear stty blkid grep awk sed dmesg ps kill uname vi modprobe insmod lsmod; do
    ln -sf busybox "$cmd"
done

# Copy KaustavOS Rust Engine
echo "=== Installing KaustavOS Rust UI ==="
cp "$UI_BIN" "$INITRAMFS_DIR/bin/kaustavos-ui"
chmod 755 "$INITRAMFS_DIR/bin/kaustavos-ui"

# Copy required dynamic libraries
echo "=== Copying Dynamic Libraries ==="
mkdir -p "$INITRAMFS_DIR/usr/lib/x86_64-linux-gnu"
mkdir -p "$INITRAMFS_DIR/lib64"

# Use ldd to find dependencies of kaustavos-ui
for lib in $(ldd "$INITRAMFS_DIR/bin/kaustavos-ui" 2>/dev/null | grep -o '/lib[^ ]*' | sort -u); do
    if [ -f "$lib" ]; then
        dir=$(dirname "$lib")
        mkdir -p "$INITRAMFS_DIR$dir"
        cp -L "$lib" "$INITRAMFS_DIR$lib"
    fi
done

# Ensure ld-linux and libc are in standard paths
if [ -f /lib64/ld-linux-x86-64.so.2 ]; then
    cp -L /lib64/ld-linux-x86-64.so.2 "$INITRAMFS_DIR/lib64/"
fi

# Copy essential filesystem kernel modules (ext4, vfat, fat, nls)
KVER="7.1.5+kali-amd64"
if [ -d "/lib/modules/$KVER" ]; then
    echo "=== Copying Kernel Filesystem Modules ($KVER) ==="
    mkdir -p "$INITRAMFS_DIR/lib/modules/$KVER/kernel/fs"
    cp -r /lib/modules/$KVER/kernel/fs/{ext4,fat,nls} "$INITRAMFS_DIR/lib/modules/$KVER/kernel/fs/" 2>/dev/null || true
    # Copy jbd2 and mbcache if separate
    find /lib/modules/$KVER/kernel -name "jbd2.ko*" -o -name "mbcache.ko*" | while read -r mod; do
        target_dir="$INITRAMFS_DIR$(dirname "$mod")"
        mkdir -p "$target_dir"
        cp "$mod" "$target_dir/"
    done
    depmod -b "$INITRAMFS_DIR" "$KVER" 2>/dev/null || true
fi

# Create /etc/passwd and /etc/group
cat << 'EOF' > "$INITRAMFS_DIR/etc/passwd"
root:x:0:0:root:/root:/bin/sh
kaustav:x:1000:1000:Kaustav Nath:/data:/bin/sh
EOF

cat << 'EOF' > "$INITRAMFS_DIR/etc/group"
root:x:0:
kaustav:x:1000:
EOF

# Create the init script
cat << 'EOF' > "$INITRAMFS_DIR/init"
#!/bin/sh
export PATH=/bin:/sbin:/usr/bin:/usr/sbin
export HOME=/data
export TERM=linux

# 1. Mount virtual filesystems
mount -t proc none /proc
mount -t sysfs none /sys
mount -t devtmpfs none /dev 2>/dev/null || mount -t tmpfs dev /dev
mkdir -p /dev/pts /dev/shm
mount -t devpts none /dev/pts 2>/dev/null || true

# 2. Suppress excessive kernel dmesg prints during startup
echo 0 > /proc/sys/kernel/printk 2>/dev/null || true

# Load filesystem drivers
modprobe ext4 2>/dev/null || true
modprobe vfat 2>/dev/null || true

# 3. Persistent USB Storage Detection
mkdir -p /data
PERSISTENT_FOUND=0

# A. Try mounting by partition label: KAUSTAV_DATA or persistence
for label in KAUSTAV_DATA persistence CASPER-RW casper-rw DATA; do
    DEV=$(blkid -L "$label" 2>/dev/null || true)
    if [ -n "$DEV" ] && [ -b "$DEV" ]; then
        if mount -o rw "$DEV" /data 2>/dev/null; then
            echo "[KaustavOS Storage] Mounted persistent volume $DEV (Label: $label) at /data"
            PERSISTENT_FOUND=1
            break
        fi
    fi
done

# B. Try finding secondary partition on USB/NVMe/SATA drive (e.g. sda2, sdb2, nvme0n1p2)
if [ "$PERSISTENT_FOUND" -eq 0 ]; then
    for part in /dev/sd[a-z]2 /dev/nvme[0-9]n[0-9]p2 /dev/mmcblk[0-9]p2; do
        if [ -b "$part" ]; then
            if mount -o rw "$part" /data 2>/dev/null; then
                echo "[KaustavOS Storage] Mounted persistent partition $part at /data"
                PERSISTENT_FOUND=1
                break
            fi
        fi
    done
fi

# C. Check if the boot USB partition itself is writable (e.g. Large FAT32 formatted by Rufus)
if [ "$PERSISTENT_FOUND" -eq 0 ]; then
    for dev in $(blkid -t TYPE=vfat -o device 2>/dev/null; blkid -t TYPE=exfat -o device 2>/dev/null); do
        if [ -b "$dev" ]; then
            mkdir -p /mnt/usb
            if mount -o rw "$dev" /mnt/usb 2>/dev/null; then
                if [ -f /mnt/usb/limine.conf ] || [ -f /mnt/usb/boot/vmlinuz ] || [ -d /mnt/usb/EFI/BOOT ]; then
                    mkdir -p /mnt/usb/kaustav_data
                    mount --bind /mnt/usb/kaustav_data /data
                    echo "[KaustavOS Storage] Mounted persistent USB FAT32 volume $dev at /data"
                    PERSISTENT_FOUND=1
                    break
                fi
                umount /mnt/usb 2>/dev/null || true
            fi
        fi
    done
fi

# D. Fallback: If no dedicated or writable USB persistent storage exists yet, use memory overlay
if [ "$PERSISTENT_FOUND" -eq 0 ]; then
    mount -t tmpfs -o size=256M tmpfs /data
    echo "[KaustavOS Storage] Using resilient volatile /data layer (No physical persistence partition found)"
fi

# Ensure storage directories
mkdir -p /data/notes /data/threads /data/config

# 4. Launch KaustavOS Themed Desktop & Shell
while true; do
    /bin/kaustavos-ui </dev/console >/dev/console 2>&1
    sync
    echo ""
    echo "KaustavOS session exited. Press Enter to restart, or type 'poweroff'."
    read -r ans </dev/console
    if [ "$ans" = "poweroff" ] || [ "$ans" = "shutdown" ]; then
        sync
        poweroff -f
    elif [ "$ans" = "reboot" ]; then
        sync
        reboot -f
    fi
done
EOF

chmod 755 "$INITRAMFS_DIR/init"

echo "=== Packing Initramfs Archive ==="
cd "$INITRAMFS_DIR"
find . -print0 | cpio --null -ov --format=newc | gzip -9 > "$BUILD_DIR/initramfs.cpio.gz"

echo "=== Initramfs created successfully at $BUILD_DIR/initramfs.cpio.gz ==="
ls -lh "$BUILD_DIR/initramfs.cpio.gz"
