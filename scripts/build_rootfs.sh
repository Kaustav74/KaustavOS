#!/bin/bash
set -e

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BUILD_DIR="$REPO_ROOT/build"
INITRAMFS_DIR="$BUILD_DIR/initramfs"

echo "=== Building KaustavOS Userspace (Rust) ==="
cd "$REPO_ROOT/userland"
cargo build --release

echo "=== Creating Initramfs Root Directory ==="
rm -rf "$INITRAMFS_DIR"
mkdir -p "$INITRAMFS_DIR"/{bin,sbin,usr/bin,usr/sbin,lib,lib64,proc,sys,dev,data,mnt,tmp,etc}

# Install BusyBox
echo "=== Installing BusyBox ==="
cp /bin/busybox "$INITRAMFS_DIR/bin/busybox"
chmod 755 "$INITRAMFS_DIR/bin/busybox"

# Create BusyBox Symlinks
cd "$INITRAMFS_DIR/bin"
for cmd in sh ls cat echo mount umount mkdir rm cp mv sync reboot poweroff sleep clear stty blkid grep awk sed dmesg ps kill uname vi; do
    ln -sf busybox "$cmd"
done

# Copy KaustavOS Rust Engine
echo "=== Installing KaustavOS Rust UI ==="
cp "$REPO_ROOT/userland/target/release/kaustavos-ui" "$INITRAMFS_DIR/bin/kaustavos-ui"
chmod 755 "$INITRAMFS_DIR/bin/kaustavos-ui"

# Copy required dynamic libraries
echo "=== Copying Dynamic Libraries ==="
mkdir -p "$INITRAMFS_DIR/usr/lib/x86_64-linux-gnu"
mkdir -p "$INITRAMFS_DIR/lib64"

# Use ldd to find dependencies of kaustavos-ui
for lib in $(ldd "$INITRAMFS_DIR/bin/kaustavos-ui" | grep -o '/lib[^ ]*' | sort -u); do
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

# C. Fallback: If no dedicated USB persistent partition exists yet, use memory overlay
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
