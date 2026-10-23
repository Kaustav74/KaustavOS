mod apps;
mod chooser;
mod shell;
mod splash;
mod storage;
mod terminal;
mod threads;

use chooser::run_chooser;
use shell::ThemedShell;
use splash::show_splash;
use storage::StorageManager;
use terminal::Terminal;
use threads::display_threads_sequence;

fn main() {
    // 1. Initialize Terminal
    Terminal::init_raw();

    // 2. Initialize Persistent Storage Manager
    let storage = StorageManager::new();

    // 3. Show Startup Splash Branding
    show_splash();

    // 4. Sequentially Display Personal Threads
    display_threads_sequence();

    // 5. Present Mode Chooser
    let chosen_theme = run_chooser(&storage);

    // 6. Launch Themed Desktop Shell
    let mut shell = ThemedShell::new(chosen_theme, storage);
    shell.run();

    // 7. Cleanup
    Terminal::restore();
}
