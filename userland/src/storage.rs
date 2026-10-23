use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OsTheme {
    Windows,
    MacOS,
    Linux,
}

impl OsTheme {
    pub fn name(&self) -> &'static str {
        match self {
            OsTheme::Windows => "Windows Style",
            OsTheme::MacOS => "macOS Style",
            OsTheme::Linux => "Linux Style",
        }
    }
}

pub struct StorageManager {
    base_dir: PathBuf,
}

impl StorageManager {
    pub fn new() -> Self {
        // Preferred persistent location is /data (mounted from USB partition or persistence image)
        let base = if Path::new("/data").exists() {
            PathBuf::from("/data")
        } else if Path::new("./kaustav_data").exists() {
            PathBuf::from("./kaustav_data")
        } else {
            let p = PathBuf::from("./kaustav_data");
            let _ = fs::create_dir_all(&p);
            p
        };

        let sm = StorageManager { base_dir: base };
        sm.ensure_directories();
        sm
    }

    fn ensure_directories(&self) {
        let _ = fs::create_dir_all(self.base_dir.join("notes"));
        let _ = fs::create_dir_all(self.base_dir.join("threads"));
        let _ = fs::create_dir_all(self.base_dir.join("config"));

        // Default initial note if none exists
        let note_path = self.base_dir.join("notes").join("welcome.txt");
        if !note_path.exists() {
            let welcome_text = "Welcome to KaustavOS Persistent Storage!\n\
                Any notes, thread comments, or settings created here will be preserved\n\
                on your USB drive across system reboots.\n";
            let _ = fs::write(note_path, welcome_text);
        }
    }

    pub fn persistence_status(&self) -> String {
        let is_usb_mount = Path::new("/data").exists();
        if is_usb_mount {
            "Active: USB Dedicated Storage Partition (/data) [Mounted Read-Write]".to_string()
        } else {
            "Testing/Emulation Storage (./kaustav_data) [Ready for USB Partition]".to_string()
        }
    }

    pub fn save_theme_preference(&self, theme: OsTheme) {
        let theme_str = match theme {
            OsTheme::Windows => "windows",
            OsTheme::MacOS => "macos",
            OsTheme::Linux => "linux",
        };
        let cfg_path = self.base_dir.join("config").join("theme.cfg");
        if let Ok(mut file) = OpenOptions::new().create(true).write(true).truncate(true).open(cfg_path) {
            let _ = file.write_all(theme_str.as_bytes());
            let _ = file.sync_all();
        }
    }

    pub fn load_theme_preference(&self) -> Option<OsTheme> {
        let cfg_path = self.base_dir.join("config").join("theme.cfg");
        if let Ok(mut file) = File::open(cfg_path) {
            let mut s = String::new();
            if file.read_to_string(&mut s).is_ok() {
                return match s.trim() {
                    "windows" => Some(OsTheme::Windows),
                    "macos" => Some(OsTheme::MacOS),
                    "linux" => Some(OsTheme::Linux),
                    _ => None,
                };
            }
        }
        None
    }

    pub fn load_notes(&self) -> String {
        let path = self.base_dir.join("notes").join("journal.txt");
        if let Ok(content) = fs::read_to_string(&path) {
            content
        } else {
            "No persistent notes yet. Type 'note <text>' in shell to add an entry!\n".to_string()
        }
    }

    pub fn append_note(&self, entry: &str) -> Result<(), String> {
        let path = self.base_dir.join("notes").join("journal.txt");
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .map_err(|e| e.to_string())?;

        writeln!(file, "• {}", entry).map_err(|e| e.to_string())?;
        file.sync_all().map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn list_files(&self) -> Vec<String> {
        let mut list = Vec::new();
        if let Ok(entries) = fs::read_dir(&self.base_dir) {
            for entry in entries.flatten() {
                if let Ok(name) = entry.file_name().into_string() {
                    list.push(name);
                }
            }
        }
        list
    }
}
