use std::io::{self, Read, Write};
use std::process::Command;

pub struct Terminal;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Key {
    Char(char),
    Enter,
    Backspace,
    Tab,
    Esc,
    Up,
    Down,
    Left,
    Right,
    None,
}

impl Terminal {
    pub fn init_raw() {
        // Switch terminal to raw mode without local echo
        let _ = Command::new("stty")
            .arg("raw")
            .arg("-echo")
            .status();
        print!("\x1b[?25l"); // Hide cursor initially
        let _ = io::stdout().flush();
    }

    pub fn restore() {
        print!("\x1b[?25h"); // Show cursor
        let _ = io::stdout().flush();
        let _ = Command::new("stty")
            .arg("sane")
            .status();
    }

    pub fn clear() {
        print!("\x1b[2J\x1b[H");
        let _ = io::stdout().flush();
    }

    pub fn move_to(row: u16, col: u16) {
        print!("\x1b[{};{}H", row, col);
        let _ = io::stdout().flush();
    }

    pub fn show_cursor(show: bool) {
        if show {
            print!("\x1b[?25h");
        } else {
            print!("\x1b[?25l");
        }
        let _ = io::stdout().flush();
    }

    pub fn read_key() -> Key {
        let mut buffer = [0u8; 4];
        let mut stdin = io::stdin();
        match stdin.read(&mut buffer) {
            Ok(0) => Key::None,
            Ok(1) => match buffer[0] {
                10 | 13 => Key::Enter,
                127 | 8 => Key::Backspace,
                9 => Key::Tab,
                27 => Key::Esc,
                b => Key::Char(b as char),
            },
            Ok(3) if buffer[0] == 27 && buffer[1] == b'[' => match buffer[2] {
                b'A' => Key::Up,
                b'B' => Key::Down,
                b'C' => Key::Right,
                b'D' => Key::Left,
                _ => Key::None,
            },
            _ => Key::None,
        }
    }
}

#[allow(dead_code)]
pub mod color {
    pub const RESET: &str = "\x1b[0m";
    pub const BOLD: &str = "\x1b[1m";
    pub const DIM: &str = "\x1b[2m";
    pub const UNDERLINE: &str = "\x1b[4m";

    // Standard foregrounds
    pub const BLACK: &str = "\x1b[30m";
    pub const RED: &str = "\x1b[31m";
    pub const GREEN: &str = "\x1b[32m";
    pub const YELLOW: &str = "\x1b[33m";
    pub const BLUE: &str = "\x1b[34m";
    pub const MAGENTA: &str = "\x1b[35m";
    pub const CYAN: &str = "\x1b[36m";
    pub const WHITE: &str = "\x1b[37m";

    // Bright foregrounds
    pub const BRIGHT_BLACK: &str = "\x1b[90m";
    pub const BRIGHT_RED: &str = "\x1b[91m";
    pub const BRIGHT_GREEN: &str = "\x1b[92m";
    pub const BRIGHT_YELLOW: &str = "\x1b[93m";
    pub const BRIGHT_BLUE: &str = "\x1b[94m";
    pub const BRIGHT_MAGENTA: &str = "\x1b[95m";
    pub const BRIGHT_CYAN: &str = "\x1b[96m";
    pub const BRIGHT_WHITE: &str = "\x1b[97m";

    // Backgrounds
    pub const BG_BLACK: &str = "\x1b[40m";
    pub const BG_RED: &str = "\x1b[41m";
    pub const BG_GREEN: &str = "\x1b[42m";
    pub const BG_YELLOW: &str = "\x1b[43m";
    pub const BG_BLUE: &str = "\x1b[44m";
    pub const BG_MAGENTA: &str = "\x1b[45m";
    pub const BG_CYAN: &str = "\x1b[46m";
    pub const BG_WHITE: &str = "\x1b[47m";

    pub const BG_DARK_GRAY: &str = "\x1b[100m";
    pub const BG_BRIGHT_BLUE: &str = "\x1b[104m";
}
