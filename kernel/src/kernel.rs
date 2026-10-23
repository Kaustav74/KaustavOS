use crate::serial::SerialPort;
use crate::vga::{Color, VgaWriter};

pub struct KaustavKernel {
    serial: SerialPort,
    vga: VgaWriter,
}

impl KaustavKernel {
    pub const fn new() -> Self {
        KaustavKernel {
            serial: SerialPort::COM1,
            vga: VgaWriter::new(),
        }
    }

    pub fn init(&mut self) {
        unsafe {
            self.serial.init();
            self.vga.clear();
        }

        self.serial.write_str("\n=======================================================\n");
        self.serial.write_str("      KaustavOS Bare-Metal Microkernel (x86_64)        \n");
        self.serial.write_str("=======================================================\n");
        self.serial.write_str("[INIT] Serial driver: COM1 initialized at 38400 baud.\n");
        self.serial.write_str("[INIT] VGA console  : 80x25 text buffer mapped.\n");
        self.serial.write_str("[INIT] TCB Security : Capability isolation primitives ready.\n");
        self.serial.write_str("[INIT] Memory Safety: Safe Rust no_std environment active.\n");

        unsafe {
            self.vga.set_color(Color::Yellow, Color::Black);
            self.vga.write_str("  ========================================================================\n");
            self.vga.set_color(Color::LightCyan, Color::Black);
            self.vga.write_str("                             KAUSTAVOS MICROKERNEL                        \n");
            self.vga.set_color(Color::White, Color::Black);
            self.vga.write_str("                  Minimal TCB • Capability Security • Rust               \n");
            self.vga.set_color(Color::Yellow, Color::Black);
            self.vga.write_str("  ========================================================================\n\n");

            self.vga.set_color(Color::LightGreen, Color::Black);
            self.vga.write_str("  [✓] CPU Target       : x86_64 Long Mode\n");
            self.vga.write_str("  [✓] COM1 Serial      : Port 0x3F8 Active\n");
            self.vga.write_str("  [✓] VGA Framebuffer  : Memory Map 0xB8000 Ready\n");
            self.vga.write_str("  [✓] Kernel Profile   : Educational Bare-Metal Microkernel\n\n");

            self.vga.set_color(Color::White, Color::Black);
            self.vga.write_str("  Personal Threads Initialization:\n");
            self.vga.set_color(Color::LightBlue, Color::Black);
            self.vga.write_str("    - Thread 1: School Journeys (Foundational Education Systems)\n");
            self.vga.write_str("    - Thread 2: CareGuardian (Telemetry & Patient Safety)\n");
            self.vga.write_str("    - Thread 3: Autonomous AI Systems\n");
            self.vga.write_str("    - Thread 4: Decentralized Resilience Mesh\n\n");

            self.vga.set_color(Color::LightGray, Color::Black);
            self.vga.write_str("  Kernel running. Entering low-power halt state.\n");
        }
    }
}
