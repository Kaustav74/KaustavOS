# Booting KaustavOS from USB with Rufus

This guide explains how to write the **KaustavOS** bootable image (`build/kaustavos.iso` or `build/kaustavos.img`) to a USB drive (e.g. 64 GB USB stick) using **Rufus** and boot it on real x86_64 PC hardware.

---

## 1. Prerequisites

1. **KaustavOS Image**:
   - `build/kaustavos.iso` (Hybrid Limine ISO with UEFI + BIOS El Torito)
   - Alternatively: `build/kaustavos.img` (Raw partitionable disk image)
2. **USB Flash Drive**:
   - 4 GB to 64 GB USB 2.0 or 3.0/3.1 flash drive.
3. **Rufus Utility** (for Windows):
   - Download the latest portable version of [Rufus](https://rufus.ie/).

---

## 2. Flashing with Rufus (Step-by-Step)

### Method A: ISO Image Mode with Persistence Slider (Recommended)

1. **Insert your USB drive** into your Windows PC.
2. **Launch Rufus**.
3. Under **Device**, select your target USB drive (verify the drive letter!).
4. Under **Boot selection**, click **SELECT** and browse to `build/kaustavos.iso`.
5. **Persistent partition size**:
   - Rufus will recognize the hybrid Linux layout and display the **Persistent partition size** slider!
   - Drag the slider to your desired storage size (e.g., **8 GB – 32 GB** on a 64 GB USB stick).
   - *Rufus automatically creates a second partition labeled `persistence` formatted with ext4!*
6. **Partition scheme**:
   - Select **GPT** (for modern UEFI PCs) or **MBR** (for older BIOS / CSM systems).
7. **Target system**:
   - `UEFI (non CSM)` or `BIOS (or UEFI-CSM)`.
8. Click **START**.
   - If prompted with *"ISOHybrid image detected"*, select **Write in ISO Image mode (Recommended)**.
9. Click **OK** to confirm drive erasure. Rufus will write the KaustavOS boot loader, kernel, initramfs, and create the persistent partition.

---

### Method B: DD Image Mode (Raw Bitstream)

If you prefer an exact byte-for-byte block clone:
1. Select your USB drive in Rufus.
2. Select `build/kaustavos.iso` (or rename/select `build/kaustavos.img`).
3. Click **START**.
4. In the ISOHybrid dialog, choose **Write in DD Image mode**.
5. Once flashing completes:
   - Open Windows **Disk Management** (`diskmgmt.msc`).
   - You will see the ~20MB KaustavOS system partition and unallocated space.
   - Right-click the unallocated space -> **New Simple Volume**.
   - Format as **ext4** (via Linux/WSL `mkfs.ext4 -L KAUSTAV_DATA /dev/sdX2`) or exFAT.
   - Label the partition: `KAUSTAV_DATA` or `persistence`.

---

## 3. Booting on Real Hardware

1. Plug the KaustavOS USB drive into your target PC or laptop.
2. Power on the PC and immediately press the **Boot Menu Key**:
   - **Dell**: `F12`
   - **Lenovo**: `F12` or `Fn + F12`
   - **HP**: `F9` or `Esc` then `F9`
   - **ASUS**: `F8` or `Esc`
   - **Acer / MSI**: `F11` or `F12`
   - **Sony Vaio**: `Assist` button or `F11`
3. In the boot menu, select the entry corresponding to your USB stick:
   - For UEFI: `UEFI: <USB Brand Name>, Partition 1`
   - For BIOS: `<USB Brand Name>`
4. **Firmware Settings Note**:
   - If Secure Boot blocks the un-enrolled custom kernel, enter BIOS setup (`F2` or `Del`) and set **Secure Boot: Disabled**.
   - Disable **Intel RST / RAID** (use AHCI/NVMe standard mode).

---

## 4. What Happens on Boot

1. **Limine Bootloader**:
   - Loads in ~1 second with the KaustavOS menu.
   - Auto-boots default entry: `KaustavOS Live (Persistent Themed Desktop & Shell)`.
2. **Kernel & Drivers**:
   - Linux x86_64 kernel initializes console, display, and USB storage.
3. **Automated Persistence Mount**:
   - Stage-1 init scans all partitions and automatically mounts the persistent partition (`persistence` or `KAUSTAV_DATA`) at `/data`.
4. **KaustavOS Startup Splash**:
   - Displays stylized "KaustavOS" branding logo with security diagnostics.
5. **Sequential Personal Threads**:
   - Briefly introduces each thread with live progress telemetry:
     - `[Thread 01] School Journeys`
     - `[Thread 02] CareGuardian`
     - `[Thread 03] AI Systems & Robotics Lab`
     - `[Thread 04] Decentralized Resilience Mesh`
6. **OS Style Chooser**:
   - Choose between:
     - `[1] Windows style` (Start menu, taskbar at bottom, classic controls)
     - `[2] macOS style` (Top menu bar, traffic-light controls, bottom Dock)
     - `[3] Linux style` (Minimal tiling status bar, hacker dark workspace)
7. **Themed Shell & Desktop**:
   - Full interactive shell with built-in commands (`help`, `threads`, `notes`, `note <text>`, `sys`, `theme <1|2|3>`, `ls`, `cat`, `reboot`, `poweroff`).
   - Every note written with `note <text>` and the selected theme are permanently saved on the USB drive!
