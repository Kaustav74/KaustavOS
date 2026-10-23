use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;
use crate::terminal::{color::*, Terminal};

pub fn show_splash() {
    Terminal::clear();
    Terminal::show_cursor(false);

    let logo = [
        r"██╗  ██╗ █████╗ ██╗   ██╗███████╗████████╗ █████╗ ██╗   ██╗ ██████╗ ███████╗",
        r"██║ ██╔╝██╔══██╗██║   ██║██╔════╝╚══██╔══╝██╔══██╗██║   ██║██╔═══██╗██╔════╝",
        r"█████╔╝ ███████║██║   ██║███████╗   ██║   ███████║██║   ██║██║   ██║███████╗",
        r"██╔═██╗ ██╔══██║██║   ██║╚════██║   ██║   ██╔══██║╚██╗ ██╔╝██║   ██║╚════██║",
        r"██║  ██╗██║  ██║╚██████╔╝███████║   ██║   ██║  ██║ ╚████╔╝ ╚██████╔╝███████║",
        r"╚═╝  ╚═╝╚═╝  ╚═╝ ╚═════╝ ╚══════╝   ╚═╝   ╚═╝  ╚═╝  ╚═══╝   ╚═════╝ ╚══════╝",
    ];

    Terminal::move_to(3, 1);
    println!("{}{}╔══════════════════════════════════════════════════════════════════════════════════╗{}", BOLD, CYAN, RESET);
    println!("{}║                                                                                  ║{}", CYAN, RESET);

    for line in &logo {
        println!("{}║   {}{:<79}{}║{}", CYAN, BOLD, line, CYAN, RESET);
    }

    println!("{}║                                                                                  ║{}", CYAN, RESET);
    println!("{}║{}                High-Assurance • Memory-Safe • Capability-Driven                {}║{}", CYAN, BRIGHT_WHITE, CYAN, RESET);
    println!("{}║{}                           Designed by Kaustav Nath                           {}║{}", CYAN, DIM, CYAN, RESET);
    println!("{}╚══════════════════════════════════════════════════════════════════════════════════╝{}", CYAN, RESET);

    Terminal::move_to(16, 8);
    print!("{}[ Hardware Initialized ]{} x86_64 UEFI/BIOS via Limine Bootloader", BRIGHT_GREEN, RESET);
    let _ = io::stdout().flush();
    sleep(Duration::from_millis(300));

    Terminal::move_to(17, 8);
    print!("{}[ Memory Protection  ]{} Safe Rust Core Active | Minimal TCB Design", BRIGHT_GREEN, RESET);
    let _ = io::stdout().flush();
    sleep(Duration::from_millis(300));

    Terminal::move_to(18, 8);
    print!("{}[ Storage Subsystem  ]{} USB Persistence Engine Ready", BRIGHT_GREEN, RESET);
    let _ = io::stdout().flush();
    sleep(Duration::from_millis(400));

    Terminal::move_to(21, 26);
    print!("{}{}Starting system threads...{}", BOLD, BRIGHT_CYAN, RESET);
    let _ = io::stdout().flush();
    sleep(Duration::from_millis(800));

    // Smooth transition
    Terminal::clear();
}
