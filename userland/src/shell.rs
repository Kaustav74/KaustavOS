use std::io::{self, BufRead, Write};
use std::process::Command;
use crate::apps::AppRunner;
use crate::storage::{OsTheme, StorageManager};
use crate::terminal::{color::*, Terminal};

pub struct ThemedShell {
    theme: OsTheme,
    storage: StorageManager,
}

impl ThemedShell {
    pub fn new(theme: OsTheme, storage: StorageManager) -> Self {
        ThemedShell { theme, storage }
    }

    pub fn run(&mut self) {
        Terminal::clear();
        Terminal::show_cursor(true);
        self.draw_desktop_frame();

        println!("Type {}'help'{} for available commands or {}'theme <1/2/3>'{} to switch desktop mode.", BOLD, RESET, BOLD, RESET);

        let stdin = io::stdin();
        let mut reader = stdin.lock();

        loop {
            self.draw_prompt();
            let mut line = String::new();
            if reader.read_line(&mut line).is_err() || line.is_empty() {
                break;
            }

            let input = line.trim();
            if input.is_empty() {
                continue;
            }

            let mut parts = input.split_whitespace();
            let cmd = parts.next().unwrap_or("").to_lowercase();
            let args: Vec<&str> = parts.collect();

            match cmd.as_str() {
                "help" | "?" => self.cmd_help(),
                "threads" | "t" => AppRunner::show_threads_view(),
                "notes" | "n" => AppRunner::show_notes(&self.storage),
                "note" => {
                    let note_text = args.join(" ");
                    AppRunner::add_note(&self.storage, &note_text);
                }
                "sys" | "sysinfo" | "info" => AppRunner::show_system_info(&self.storage),
                "ls" | "dir" | "f" | "files" => AppRunner::list_files(&self.storage),
                "cat" => {
                    if let Some(filename) = args.first() {
                        let path = format!("/data/{}", filename);
                        let local_path = format!("./kaustav_data/{}", filename);
                        if let Ok(c) = std::fs::read_to_string(&path).or_else(|_| std::fs::read_to_string(&local_path)) {
                            println!("\n--- {} ---\n{}\n", filename, c);
                        } else {
                            println!("{}Cannot open file: {}{}", RED, filename, RESET);
                        }
                    } else {
                        println!("Usage: cat <filename>");
                    }
                }
                "theme" => {
                    if let Some(choice) = args.first() {
                        match *choice {
                            "1" | "win" | "windows" => self.switch_theme(OsTheme::Windows),
                            "2" | "mac" | "macos" => self.switch_theme(OsTheme::MacOS),
                            "3" | "lin" | "linux" => self.switch_theme(OsTheme::Linux),
                            _ => println!("Invalid theme. Choose: 1 (Windows), 2 (macOS), 3 (Linux)"),
                        }
                    } else {
                        println!("Current theme: {}. Change with 'theme 1', 'theme 2', 'theme 3'", self.theme.name());
                    }
                }
                "start" | "s" => {
                    if self.theme == OsTheme::Windows {
                        self.show_windows_start_menu();
                    } else {
                        println!("Start menu is unique to Windows theme. Switch via 'theme 1'.");
                    }
                }
                "dock" => {
                    if self.theme == OsTheme::MacOS {
                        self.show_macos_dock();
                    } else {
                        println!("Dock is unique to macOS theme. Switch via 'theme 2'.");
                    }
                }
                "clear" | "cls" => {
                    Terminal::clear();
                    self.draw_desktop_frame();
                }
                "persist" | "sync" => {
                    println!("\n{}{}Persisting data...{}", BOLD, CYAN, RESET);
                    let _ = Command::new("sync").status();
                    println!("{}{}Persistence status: {}{}", BOLD, GREEN, self.storage.persistence_status(), RESET);
                }
                "reboot" => {
                    println!("\n{}{}Syncing persistent storage and rebooting system...{}", BOLD, YELLOW, RESET);
                    let _ = Command::new("sync").status();
                    let _ = Command::new("reboot").status();
                    break;
                }
                "poweroff" | "shutdown" | "exit" | "quit" => {
                    println!("\n{}{}Syncing storage and powering off...{}", BOLD, RED, RESET);
                    let _ = Command::new("sync").status();
                    let _ = Command::new("poweroff").status();
                    break;
                }
                unknown => {
                    println!("Unknown command: '{}'. Type 'help' for command list.", unknown);
                }
            }
        }
    }

    fn switch_theme(&mut self, new_theme: OsTheme) {
        self.theme = new_theme;
        self.storage.save_theme_preference(new_theme);
        Terminal::clear();
        self.draw_desktop_frame();
        println!("{}{}Switched to {}. Setting persisted to USB!{}", BOLD, BRIGHT_GREEN, self.theme.name(), RESET);
    }

    fn draw_desktop_frame(&self) {
        match self.theme {
            OsTheme::Windows => {
                println!("{}{}  KaustavOS Professional  [Windows Edition]                                🛡 TCB: Min  {}  ", BG_BLUE, BRIGHT_WHITE, RESET);
                println!("{}{}┌─ [ 🗔 KaustavOS Command Center ] ────────────────────────────── [ _ ] [ □ ] [ ✕ ] ─┐{}", CYAN, BOLD, RESET);
                println!("{}│  Persistence: Active  │  Theme: Windows Style  │  Security: Rust Memory Safe     │{}", CYAN, RESET);
                println!("{}└───────────────────────────────────────────────────────────────────────────────────┘{}", CYAN, RESET);
                println!("{} Taskbar: [ 🪟 Start (s) ] [ 📁 Files (f) ] [ 📝 Notes (n) ] [ 🧬 Threads (t) ]     {}", BG_DARK_GRAY, RESET);
                println!();
            }
            OsTheme::MacOS => {
                println!("{}{}   KaustavOS   File   Edit   View   Threads   Window   Help       🔋 100%   📶 USB   {}  ", BG_DARK_GRAY, BRIGHT_WHITE, RESET);
                println!("{}{}┌─ [● ● ●]  🗂 KaustavOS Workspace (macOS Edition) ──────────────────────────────────┐{}", WHITE, BOLD, RESET);
                println!("{}│  Local-First Mesh     │  Theme: macOS Style    │  Capability Isolation: Enforced  │{}", WHITE, RESET);
                println!("{}└───────────────────────────────────────────────────────────────────────────────────┘{}", WHITE, RESET);
                println!("{}   Dock: [ Finder (f) ] [ Notes (n) ] [ Threads (t) ] [ Terminal ] [ Prefs ]        {}", BG_BLACK, RESET);
                println!();
            }
            OsTheme::Linux => {
                println!("{}{} [1:term] [2:threads] [3:notes] [4:sys] │ kaustav@kaustavos:x86_64 │ TCB: Minimal   {}  ", BG_BLACK, BRIGHT_GREEN, RESET);
                println!("{}{}┌─ [ terminal : /data ] ────────────────────────────────────────────── [mode: tiling] ─┐{}", GREEN, BOLD, RESET);
                println!("{}│  Zero-bloat base      │  Theme: Linux Style    │  Capability-based sandbox       │{}", GREEN, RESET);
                println!("{}└───────────────────────────────────────────────────────────────────────────────────┘{}", GREEN, RESET);
                println!();
            }
        }
    }

    fn draw_prompt(&self) {
        match self.theme {
            OsTheme::Windows => {
                print!("{}C:\\KaustavOS\\Data>{} ", BRIGHT_CYAN, RESET);
            }
            OsTheme::MacOS => {
                print!("{}kaustav@MacBook-KaustavOS /data %{} ", BRIGHT_WHITE, RESET);
            }
            OsTheme::Linux => {
                print!("{}{}kaustav@kaustavos{}:{}{}/data{}$ ", BOLD, BRIGHT_GREEN, RESET, BOLD, BRIGHT_CYAN, RESET);
            }
        }
        let _ = io::stdout().flush();
    }

    fn show_windows_start_menu(&self) {
        println!("\n{}{}┌── [ 🪟 Windows Start Menu ] ─────────────┐{}", BG_BLUE, BRIGHT_WHITE, RESET);
        println!("│ [1] Personal Threads (type 'threads')    │");
        println!("│ [2] Persistent Notes (type 'notes')      │");
        println!("│ [3] System Telemetry (type 'sys')        │");
        println!("│ [4] File Explorer    (type 'ls')         │");
        println!("│ [5] Switch OS Style  (type 'theme <1-3>')│");
        println!("│ [6] Shutdown System  (type 'poweroff')   │");
        println!("└──────────────────────────────────────────┘\n");
    }

    fn show_macos_dock(&self) {
        println!("\n{}{}╭── [  macOS Application Dock ] ─────────────╮{}", BG_DARK_GRAY, BRIGHT_WHITE, RESET);
        println!("│ 📁 Finder      (type 'ls')                  │");
        println!("│ 📝 Notes       (type 'notes')               │");
        println!("│ 🧬 Threads     (type 'threads')             │");
        println!("│ ⚙ Settings    (type 'theme')               │");
        println!("╰─────────────────────────────────────────────╯\n");
    }

    fn cmd_help(&self) {
        println!("\n{}{}=== KAUSTAVOS COMMAND REFERENCE ==={}", BOLD, CYAN, RESET);
        println!("  {}threads, t{}      : View personal threads (School Journeys, CareGuardian, etc.)", BRIGHT_YELLOW, RESET);
        println!("  {}notes, n{}        : Display persistent notes from USB storage", BRIGHT_YELLOW, RESET);
        println!("  {}note <text>{}     : Save a new entry to the persistent USB journal", BRIGHT_YELLOW, RESET);
        println!("  {}ls, files, f{}    : List persistent storage directory (/data)", BRIGHT_YELLOW, RESET);
        println!("  {}cat <filename>{}  : Read content of a stored file", BRIGHT_YELLOW, RESET);
        println!("  {}sys, sysinfo{}    : Inspect hardware, memory, TCB security, and persistence", BRIGHT_YELLOW, RESET);
        println!("  {}theme <1|2|3>{}   : Hot-switch desktop UI [1=Windows, 2=macOS, 3=Linux]", BRIGHT_YELLOW, RESET);
        println!("  {}start, s{}        : Open Windows Start Menu (Windows theme)", BRIGHT_YELLOW, RESET);
        println!("  {}dock{}            : Open macOS Dock Launcher (macOS theme)", BRIGHT_YELLOW, RESET);
        println!("  {}persist, sync{}   : Verify USB persistent storage and flush write buffers", BRIGHT_YELLOW, RESET);
        println!("  {}clear, cls{}      : Redraw the themed desktop interface", BRIGHT_YELLOW, RESET);
        println!("  {}reboot{}          : Safely flush storage and reboot machine", BRIGHT_YELLOW, RESET);
        println!("  {}poweroff, exit{}  : Safely flush storage and turn off power\n", BRIGHT_YELLOW, RESET);
    }
}
