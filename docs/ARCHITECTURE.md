# KaustavOS Architecture

## Core Design
Inspired by seL4 (formally verified microkernel) and modern Rust OS projects (e.g., framekernel ideas, Redox).

### Kernel (Privileged)
- Extremely small.
- Provides:
  - Thread scheduling
  - Capability-based access control
  - Synchronous IPC (endpoints)
  - Address space management primitives
  - Interrupt handling
- No heap after boot (user-level resource managers supply memory).
- Written in safe Rust where possible; unsafe confined to tiny audited modules.

### User Space
- Everything else: file system, network, drivers, GUI, application servers.
- Each runs in its own isolated address space.
- Communicate only via capabilities and IPC.
- A compromise in one service cannot freely attack the kernel or other domains.

### Boot Experience
1. Bootloader loads kernel from USB.
2. Kernel initializes hardware minimally.
3. Early userspace or kernel stub shows splash:
   - "KaustavOS"
   - Sequential thread previews (School Journeys, CareGuardian, …) that fade.
4. Mode selector: Windows / macOS / Linux style.
5. Launch corresponding UI shell (themed desktop) with full persistence on the USB partition.

### Persistence
- Dedicated partition or file system on the 64 GB USB for user data, settings, and threads.
- All changes survive reboots.

### Security Goals
- Minimize attack surface.
- Isolate untrusted code.
- Prefer formal methods and language safety over trusting large C codebases.
- Practical path: start with Rust microkernel skeleton, grow verified components over time.

This design prioritizes security and lightness while allowing rich functionality through user-space composition.
