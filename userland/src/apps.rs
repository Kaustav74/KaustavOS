use crate::storage::StorageManager;
use crate::terminal::color::*;
use crate::threads::PERSONAL_THREADS;
use std::fs;

pub struct AppRunner;

impl AppRunner {
    pub fn show_threads_view() {
        println!("\n{}{}=== KAUSTAVOS PERSONAL THREADS REGISTRY ==={}", BOLD, CYAN, RESET);
        for t in PERSONAL_THREADS {
            println!(" {}{}[Thread {:02}] {}{}", BOLD, BRIGHT_YELLOW, t.id, t.title, RESET);
            println!("   Category : {}", t.category);
            println!("   Summary  : {}", t.description);
            println!("   Telemetry: {}{}{}", BRIGHT_GREEN, t.telemetry, RESET);
            println!("   Tag      : {}{}{}\n", BRIGHT_MAGENTA, t.tag, RESET);
        }
    }

    pub fn show_notes(storage: &StorageManager) {
        println!("\n{}{}=== PERSISTENT JOURNAL (USB Storage) ==={}", BOLD, YELLOW, RESET);
        let content = storage.load_notes();
        println!("{}", content);
    }

    pub fn add_note(storage: &StorageManager, text: &str) {
        if text.trim().is_empty() {
            println!("{}Note text cannot be empty. Usage: note <your text>{}", RED, RESET);
            return;
        }
        match storage.append_note(text) {
            Ok(_) => println!("{}{}✔ Note successfully saved and synced to persistent USB storage!{}", BOLD, BRIGHT_GREEN, RESET),
            Err(e) => println!("{}{}Failed to save note: {}{}", BOLD, RED, e, RESET),
        }
    }

    pub fn show_system_info(storage: &StorageManager) {
        println!("\n{}{}=== KAUSTAVOS SYSTEM & SECURITY TELEMETRY ==={}", BOLD, BRIGHT_CYAN, RESET);
        println!("  Target Architecture  : x86_64 (Intel/AMD)");
        println!("  Core Runtime         : Safe Rust 1.95 (Zero Memory Corruptions)");
        println!("  Bootloader           : Limine v8.x (UEFI + BIOS Hybrid El Torito)");
        println!("  Kernel Base          : Minimal TCB Micro-Profile (Linux/Rust Hybrid)");
        println!("  Persistence Status   : {}", storage.persistence_status());

        // Read /proc/meminfo or /proc/version if available
        if let Ok(version) = fs::read_to_string("/proc/version") {
            let line = version.lines().next().unwrap_or("");
            println!("  Underlying Kernel    : {}", line);
        }
        if let Ok(meminfo) = fs::read_to_string("/proc/meminfo") {
            let mem_total = meminfo.lines().find(|l| l.starts_with("MemTotal")).unwrap_or("");
            let mem_free = meminfo.lines().find(|l| l.starts_with("MemAvailable")).unwrap_or("");
            println!("  Memory Footprint     : {} | {}", mem_total.trim(), mem_free.trim());
        }
        println!("  Security Perimeter   : Capability-isolated userspace; minimal privilege attack surface.\n");
    }

    pub fn list_files(storage: &StorageManager) {
        println!("\n{}{}=== PERSISTENT STORAGE CONTENTS (/data) ==={}", BOLD, BRIGHT_WHITE, RESET);
        let files = storage.list_files();
        if files.is_empty() {
            println!("  (No files found)");
        } else {
            for f in files {
                println!("  📁 {}", f);
            }
        }
        println!();
    }
}
