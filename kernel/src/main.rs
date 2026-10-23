#![no_std]
#![no_main]

use core::panic::PanicInfo;

/// Simple KaustavOS kernel entry.
/// This is a minimal placeholder. Replace the loop with real hardware init
/// (serial, VGA, framebuffer) using public educational tutorials.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // === Boot Sequence (to be implemented) ===
    // 1. Initialize early console (serial or VGA text mode)
    // 2. Clear screen and print:
    //    ====================================
    //              KaustavOS
    //    ====================================
    //
    // 3. Briefly show threads then clear:
    //    School Journeys
    //    CareGuardian
    //    (add more of your own)
    //
    // 4. Show simple chooser menu:
    //    Use like which OS?
    //    [1] Windows style
    //    [2] macOS style
    //    [3] Linux style
    //
    // 5. Based on choice, load a themed shell (future work)
    //    and enable persistence on USB.

    // For now: infinite loop (kernel stays alive)
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
