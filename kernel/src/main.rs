#![no_std]
#![no_main]

use core::panic::PanicInfo;

/// Entry point of the KaustavOS kernel.
/// In a real build this would be linked with a bootloader (Limine/GRUB)
/// and run in QEMU or on real hardware from USB.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Placeholder boot sequence simulation
    // In the full system this would drive a framebuffer or serial console

    // 1. Show KaustavOS branding
    // (would print or render "KaustavOS")

    // 2. Show personal threads that then disappear
    // School Journeys, CareGuardian, etc.

    // 3. Present OS mode chooser: Windows / macOS / Linux

    // 4. Launch the selected interface shell with persistence

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
