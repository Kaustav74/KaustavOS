# KaustavOS

**KaustavOS** is a lightweight, high-assurance operating system project aiming for maximum security and minimal trusted computing base (TCB).

## Vision
- Extremely small privileged kernel (microkernel style, inspired by seL4).
- Capability-based access control and strong isolation.
- Memory-safe core written in Rust (no_std bare metal).
- Boots from a USB pendrive (64 GB, USB 2.0 compatible).
- Custom startup experience:
  1. Shows **KaustavOS** branding.
  2. Briefly displays personal threads (e.g., School Journeys, CareGuardian, etc.) then fades.
  3. Presents a chooser: "Use like which OS: Windows, macOS or Linux".
  4. After selection, provides a consistent interface matching the chosen style while keeping the secure KaustavOS foundation underneath.
- Persistent data storage on the USB.

## Architecture Principles (Safest Practical)
- **Minimal TCB**: Only essential mechanisms in privileged mode (scheduling, IPC, memory primitives, capability system).
- Drivers, filesystems, networking, and GUI run as isolated user-space servers.
- Formal verification goals where feasible (start with rigorous design and Rust safety).
- Principle of least privilege.
- Hardware MMU + IOMMU isolation.

This is the closest achievable to "almost unhackable" for software-based attacks: common kernel exploits are minimized by design and language safety.

## Current Status
This repository contains the **initial skeleton**. A full production OS with complete GUI compatibility layers for Windows/macOS/Linux interfaces is a multi-year effort. This project provides the secure foundation and boot structure to build upon.

## Building & Running (Planned)
- Target: x86_64 bare metal (later ARM/RISC-V).
- Bootloader: Limine or GRUB for USB live boot.
- Build with Rust nightly + cargo.
- Test in QEMU first, then write to USB.

See `docs/ARCHITECTURE.md` for detailed design notes.

## Roadmap
1. Minimal bootable kernel that prints "KaustavOS".
2. Custom splash + thread preview sequence.
3. Basic capability system and IPC.
4. USB persistence layer.
5. Desktop mode chooser and themed UI shells.
6. User-space services and isolation.

Created by Kaustav Nath (Kaustav74).
