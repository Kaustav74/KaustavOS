use crate::storage::{OsTheme, StorageManager};
use crate::terminal::{color::*, Key, Terminal};

pub fn run_chooser(storage: &StorageManager) -> OsTheme {
    Terminal::clear();
    Terminal::show_cursor(false);

    let saved_theme = storage.load_theme_preference();

    Terminal::move_to(3, 1);
    println!("{}{}╔══════════════════════════════════════════════════════════════════════════════════╗{}", BOLD, CYAN, RESET);
    println!("{}║                         KAUSTAVOS DESKTOP STYLE CHOOSER                          ║{}", CYAN, RESET);
    println!("{}╚══════════════════════════════════════════════════════════════════════════════════╝{}", CYAN, RESET);

    Terminal::move_to(6, 4);
    println!("{}Select how you would like to experience KaustavOS:{}", BOLD, RESET);

    // Option 1: Windows Style
    Terminal::move_to(8, 4);
    println!("{}{} [1] Windows Style Experience {}{}", BG_BLUE, BRIGHT_WHITE, RESET, RESET);
    Terminal::move_to(9, 8);
    println!("{}• Taskbar positioned at bottom with [Start] button & system clock{}", CYAN, RESET);
    Terminal::move_to(10, 8);
    println!("{}• Classic window decorations with right-aligned [_] [□] [X] controls{}", CYAN, RESET);
    Terminal::move_to(11, 8);
    println!("{}• Blue/slate theme palette with familiar desktop shortcuts{}", CYAN, RESET);

    // Option 2: macOS Style
    Terminal::move_to(13, 4);
    println!("{}{} [2] macOS Style Experience {}{}", BG_DARK_GRAY, BRIGHT_WHITE, RESET, RESET);
    Terminal::move_to(14, 8);
    println!("{}• Global top menu bar ( KaustavOS, File, Edit, Status indicators){}", BRIGHT_WHITE, RESET);
    Terminal::move_to(15, 8);
    println!("{}• Modern traffic-light window controls [● ● ●] on top-left{}", BRIGHT_WHITE, RESET);
    Terminal::move_to(16, 8);
    println!("{}• Bottom centered application Dock with quick launchers{}", BRIGHT_WHITE, RESET);

    // Option 3: Linux Style
    Terminal::move_to(18, 4);
    println!("{}{} [3] Linux Style Experience {}{}", BG_BLACK, BRIGHT_GREEN, RESET, RESET);
    Terminal::move_to(19, 8);
    println!("{}• Minimalist tiling / status workspace bar [1:term] [2:threads] [3:sys]{}", GREEN, RESET);
    Terminal::move_to(20, 8);
    println!("{}• High-contrast hacker dark theme with zero-bloat CLI efficiency{}", GREEN, RESET);
    Terminal::move_to(21, 8);
    println!("{}• Direct capability shell and terminal-first workflow{}", GREEN, RESET);

    Terminal::move_to(23, 4);
    if let Some(saved) = saved_theme {
        println!("{}{}Saved preference detected: {} (Press ENTER to continue, or [1/2/3] to change){}", DIM, YELLOW, saved.name(), RESET);
    } else {
        println!("{}{}Press [1], [2], or [3] to select your style:{}", BOLD, BRIGHT_YELLOW, RESET);
    }

    loop {
        match Terminal::read_key() {
            Key::Char('1') => {
                storage.save_theme_preference(OsTheme::Windows);
                return OsTheme::Windows;
            }
            Key::Char('2') => {
                storage.save_theme_preference(OsTheme::MacOS);
                return OsTheme::MacOS;
            }
            Key::Char('3') => {
                storage.save_theme_preference(OsTheme::Linux);
                return OsTheme::Linux;
            }
            Key::Enter => {
                let theme = saved_theme.unwrap_or(OsTheme::Windows);
                storage.save_theme_preference(theme);
                return theme;
            }
            _ => {}
        }
    }
}
