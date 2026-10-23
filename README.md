# KaustavOS

**A practical, bootable, lightweight operating system image built with safe Rust and the Limine bootloader, ready to flash with Rufus and boot on real x86_64 hardware.**

---

## 🌟 Key Highlights

- 🚀 **Rufus USB Ready**: Generates a standard hybrid ISO/IMG (`build/kaustavos.iso`) supporting both **UEFI** and **Legacy BIOS** booting.
- 🎨 **Startup Branding Splash**: Large, stylized ANSI "KaustavOS" branding logo with hardware initialization and memory protection telemetry.
- 🧬 **Sequential Personal Threads**: Smoothly displays personal workspace threads in sequence with live progress animations, then transitions into the desktop:
  1. **School Journeys** (Foundational education & algorithms)
  2. **CareGuardian** (Health telemetry & guardian systems)
  3. **AI Systems & Robotics Lab** (Autonomous edge runtimes)
  4. **Decentralized Resilience Mesh** (Local-first capability security)
- 🖥️ **Three Desktop Style Experiences**:
  - **[1] Windows style**: Bottom taskbar with `[Start]` button, classic blue palette, and right-aligned window buttons `[_] [□] [X]`.
  - **[2] macOS style**: Global top menu bar (` KaustavOS`), left-aligned traffic lights `[● ● ●]`, and bottom centered application Dock.
  - **[3] Linux style**: Minimal tiling status bar `[1:term] [2:threads] [3:sys]`, dark hacker palette, and direct terminal shell.
- 💾 **USB Data Persistence**: User settings (saved theme preference), notes, and thread data are automatically saved to a persistent partition (`KAUSTAV_DATA` or `persistence`) and survive across reboots!
- 🛡️ **High-Assurance Architecture**: Minimal Trusted Computing Base (TCB) (~20 MB total footprint) written with memory-safe **Rust 1.95**.
- 🧪 **Dual Architecture**: Includes both the production-ready **KaustavOS Live System** and the educational **Bare-Metal Rust Microkernel** skeleton in `kernel/`.

---

## 📂 Repository Structure

```
KaustavOS-main/
├── Cargo.toml                  # Workspace root configuration
├── Makefile                    # One-command build and test targets
├── run_qemu.bat                # Windows one-click launcher
├── bootloader/                 # Limine bootloader binaries and config
│   ├── limine.conf             # Limine boot configuration
│   └── limine/                 # Limine UEFI & BIOS boot files & host tool
├── userland/                   # Safe Rust production system & shell
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs             # Boot splash, threads, chooser orchestrator
│       ├── splash.rs           # Branding logo and diagnostics
│       ├── threads.rs          # Personal threads sequence & telemetry
│       ├── chooser.rs          # Windows / macOS / Linux mode selector
│       ├── shell.rs            # Themed desktops & command center
│       ├── apps.rs             # Built-in applications (Notes, Threads, Sysinfo)
│       ├── storage.rs          # USB persistence manager & sync engine
│       └── terminal.rs         # ANSI terminal & raw keyboard driver
├── kernel/                     # Bare-metal no_std Rust microkernel skeleton
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs             # Microkernel entry point
│       ├── kernel.rs           # Microkernel init and hardware status
│       ├── serial.rs           # COM1 (0x3F8) serial port driver
│       └── vga.rs              # 80x25 text mode framebuffer driver
├── scripts/                    # Build and packaging automation
│   ├── build_iso.sh            # Builds hybrid bootable ISO & IMG
│   ├── build_rootfs.sh         # Compiles Rust UI & packages initramfs
│   ├── run_qemu.sh             # Launches QEMU with persistence
│   └── test_interactive.sh     # Automated QEMU integration test
└── docs/                       # Comprehensive documentation
    ├── ARCHITECTURE.md         # TCB security, capability model, and trade-offs
    ├── BOOT_USB.md             # Detailed Rufus flashing and BIOS/UEFI boot guide
    └── SIMPLE_START.md         # Quickstart guide
```

---

## ⚡ Quickstart

### 1. Build the Bootable Image
In WSL or Linux (requires `cargo`, `xorriso`, `make`):

```bash
make iso
```

This compiles `kaustavos-ui`, packages the initramfs, and uses Limine with xorriso to create:
- `build/kaustavos.iso` (~20 MB hybrid ISO)
- `build/kaustavos.img` (raw disk image)

### 2. Test in QEMU

#### On Windows:
Double-click `run_qemu.bat` or run:
```cmd
run_qemu.bat
```

#### On Linux / WSL:
```bash
make run
```
Or for headless/terminal mode:
```bash
make run-cli
```

### 3. Run Automated Integration Tests
Verify the entire boot, splash, thread animation, chooser, and persistence flow in QEMU:
```bash
make test
```

---

## 💾 Writing to USB with Rufus (Real Hardware Boot)

1. Open **Rufus** on Windows and plug in your USB drive (4 GB to 64 GB).
2. Under **Boot selection**, select `build/kaustavos.iso`.
3. Under **Persistent partition size**, drag the slider to your desired persistence size (e.g., **8 GB – 32 GB** on a 64 GB USB drive). Rufus will automatically create a dedicated ext4 `persistence` partition.
4. Partition scheme: **GPT** (UEFI) or **MBR** (BIOS/CSM).
5. Click **START** -> Choose **Write in ISO Image mode (Recommended)**.
6. Insert the USB drive into your PC, reboot, press your boot menu key (`F12`, `F9`, or `F8`), and select the USB drive!

*For detailed instructions and troubleshooting, see [docs/BOOT_USB.md](file:///d:/KaustavOS/KaustavOS-main/KaustavOS-main/docs/BOOT_USB.md).*

---

## 💻 Interactive Themed Desktop Shell

Once booted, KaustavOS provides an interactive command center tailored to your chosen theme:

| Command | Action |
| :--- | :--- |
| `threads` (or `t`) | View personal threads registry and telemetry |
| `notes` (or `n`) | View persistent notes loaded from USB storage |
| `note <text>` | Append a note and **synchronously write it to USB persistence** |
| `theme <1\|2\|3>` | Hot-swap desktop UI (`1`=Windows, `2`=macOS, `3`=Linux) |
| `sys` (or `sysinfo`)| Inspect hardware, memory footprint, and security perimeter |
| `ls` (or `files`, `f`)| List files saved in persistent storage (`/data`) |
| `cat <filename>` | View the contents of any saved file |
| `start` (or `s`) | Open the Windows Start Menu popup (Windows theme) |
| `dock` | Open macOS Dock launcher (macOS theme) |
| `persist` | Verify USB persistent partition health and flush disk buffers |
| `reboot` | Safely sync storage and reboot hardware |
| `poweroff` | Safely sync storage and power off machine |

---

## 🛡️ Architecture & Security Principles

KaustavOS is designed according to high-assurance engineering practices:
1. **Minimal TCB**: No bloated daemons or unauthenticated background network stacks.
2. **Memory Safety**: Written in safe Rust 1.95, eliminating buffer overflows, use-after-free, and data races.
3. **Capability-Style Isolation**: Storage and subsystems communicate through explicit interfaces.
4. **Reliable USB Persistence**: Automatically discovers partitions labeled `persistence` or `KAUSTAV_DATA` with fallback to in-memory tmpfs.

For complete architectural notes, design decisions, and trade-off analyses, see [docs/ARCHITECTURE.md](file:///d:/KaustavOS/KaustavOS-main/KaustavOS-main/docs/ARCHITECTURE.md).

---

## 👤 Author
Created by **Kaustav Nath**.
