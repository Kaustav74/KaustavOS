use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;
use crate::terminal::{color::*, Terminal};

pub struct ThreadInfo {
    pub id: usize,
    pub title: &'static str,
    pub category: &'static str,
    pub description: &'static str,
    pub telemetry: &'static str,
    pub tag: &'static str,
}

pub const PERSONAL_THREADS: &[ThreadInfo] = &[
    ThreadInfo {
        id: 1,
        title: "School Journeys",
        category: "Foundations & Education Architecture",
        description: "Formative learning pathways, foundational algorithms, and educational computing.",
        telemetry: "Milestones: 100% | Archival Record: Verified | Integrity: Signed",
        tag: "CORE_EDUCATION",
    },
    ThreadInfo {
        id: 2,
        title: "CareGuardian",
        category: "Critical Health Telemetry & Guardian Systems",
        description: "Adaptive patient monitoring, edge telemetry, and emergency caregiver safety nets.",
        telemetry: "Heartbeat: Active | Low-Latency Dispatch: Online | Fault-Tolerance: TCB-1",
        tag: "HEALTH_GUARDIAN",
    },
    ThreadInfo {
        id: 3,
        title: "AI Systems & Robotics Lab",
        category: "Autonomous Edge Runtimes & Intelligence",
        description: "Agentic pairing frameworks, micro-runtimes, and resilient real-time robotics control.",
        telemetry: "Model Runtime: Optimized | IPC Channels: Bound | Latency: 1.2ms",
        tag: "AUTONOMOUS_AI",
    },
    ThreadInfo {
        id: 4,
        title: "Decentralized Resilience Mesh",
        category: "Local-First Security & Capability Mesh",
        description: "Zero-trust local-first storage, encrypted journals, and capability-based partitions.",
        telemetry: "Vault: AES-GCM Encrypted | USB Persistence: Synced | Zero-Cloud Mode",
        tag: "SECURITY_MESH",
    },
];

pub fn display_threads_sequence() {
    Terminal::clear();
    Terminal::show_cursor(false);

    Terminal::move_to(2, 1);
    println!("{}{}╔══════════════════════════════════════════════════════════════════════════════════╗{}", BOLD, BRIGHT_BLUE, RESET);
    println!("{}║                     KAUSTAVOS PERSONAL THREADS INITIALIZER                       ║{}", BRIGHT_BLUE, RESET);
    println!("{}╚══════════════════════════════════════════════════════════════════════════════════╝{}", BRIGHT_BLUE, RESET);

    println!("\n  {}Synchronizing personal workspace threads in sequence...{}\n", DIM, RESET);

    for thread in PERSONAL_THREADS {
        // Draw thread box
        Terminal::move_to(7, 1);
        println!("  {}{}┌── [Thread {:02}/04] : {} ──────────────────────────────────────┐{}", BOLD, CYAN, thread.id, thread.title, RESET);
        println!("  {}│{} Domain      : {}{:<58}{}│{}", CYAN, RESET, BRIGHT_YELLOW, thread.category, CYAN, RESET);
        println!("  {}│{} Description : {}{:<58}{}│{}", CYAN, RESET, WHITE, thread.description, CYAN, RESET);
        println!("  {}│{} Telemetry   : {}{:<58}{}│{}", CYAN, RESET, BRIGHT_GREEN, thread.telemetry, CYAN, RESET);
        println!("  {}│{} Capability  : {}{:<58}{}│{}", CYAN, RESET, BRIGHT_MAGENTA, thread.tag, CYAN, RESET);
        println!("  {}└────────────────────────────────────────────────────────────────────────┘{}", CYAN, RESET);

        // Progress bar simulation for each thread
        Terminal::move_to(14, 5);
        print!("{}Loading thread state: [", BRIGHT_WHITE);
        for _step in 0..20 {
            print!("{}■{}", BRIGHT_CYAN, RESET);
            let _ = io::stdout().flush();
            sleep(Duration::from_millis(30));
        }
        println!("] {}READY{}", BRIGHT_GREEN, RESET);
        let _ = io::stdout().flush();

        sleep(Duration::from_millis(600));

        // Fade/Clear thread display region for next thread
        for line in 7..=15 {
            Terminal::move_to(line, 1);
            print!("                                                                                    ");
        }
        let _ = io::stdout().flush();
    }

    Terminal::move_to(8, 15);
    println!("{}{}✔ All personal threads successfully loaded and registered.{}", BOLD, BRIGHT_GREEN, RESET);
    Terminal::move_to(10, 18);
    println!("{}Transitioning to desktop environment chooser...{}", DIM, RESET);
    let _ = io::stdout().flush();
    sleep(Duration::from_millis(900));

    Terminal::clear();
}
