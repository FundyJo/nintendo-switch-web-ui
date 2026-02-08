use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use base64::{Engine as _, engine::general_purpose};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Game {
    pub id: String,
    pub title: String,
    pub path: PathBuf,
    pub icon: Option<String>, // Base64 encoded icon
    pub emulator: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameScanner {
    pub games: Vec<Game>,
}

impl GameScanner {
    pub fn new() -> Self {
        GameScanner {
            games: Vec::new(),
        }
    }

    /// Scan for Yuzu games
    pub fn scan_yuzu(&mut self) -> Result<(), String> {
        if let Some(home_dir) = dirs::home_dir() {
            // Typical Yuzu game directory locations
            let yuzu_paths = vec![
                home_dir.join(".local/share/yuzu/load"),
                home_dir.join("AppData/Roaming/yuzu/load"),
                home_dir.join("Library/Application Support/yuzu/load"),
            ];

            for yuzu_path in yuzu_paths {
                if yuzu_path.exists() {
                    self.scan_directory(&yuzu_path, "yuzu")?;
                }
            }
        }
        Ok(())
    }

    /// Scan for Ryujinx games
    pub fn scan_ryujinx(&mut self) -> Result<(), String> {
        if let Some(home_dir) = dirs::home_dir() {
            // Typical Ryujinx game directory locations
            let ryujinx_paths = vec![
                home_dir.join(".config/Ryujinx/games"),
                home_dir.join("AppData/Roaming/Ryujinx/games"),
                home_dir.join("Library/Application Support/Ryujinx/games"),
            ];

            for ryujinx_path in ryujinx_paths {
                if ryujinx_path.exists() {
                    self.scan_directory(&ryujinx_path, "ryujinx")?;
                }
            }
        }
        Ok(())
    }

    /// Scan a directory for Switch game files
    fn scan_directory(&mut self, path: &Path, emulator: &str) -> Result<(), String> {
        // Look for .nsp, .xci, .nca, .nro files
        let game_extensions = vec!["nsp", "xci", "nca", "nro"];
        
        for entry in WalkDir::new(path)
            .max_depth(3)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if let Some(ext) = path.extension() {
                if game_extensions.contains(&ext.to_str().unwrap_or("")) {
                    if let Some(file_name) = path.file_stem() {
                        let title = file_name.to_string_lossy().to_string();
                        let id = format!("{:x}", md5::compute(path.to_string_lossy().as_bytes()));
                        
                        let icon = self.find_game_icon(path);
                        
                        self.games.push(Game {
                            id,
                            title,
                            path: path.to_path_buf(),
                            icon,
                            emulator: emulator.to_string(),
                        });
                    }
                }
            }
        }
        Ok(())
    }

    /// Try to find an icon for the game
    fn find_game_icon(&self, game_path: &Path) -> Option<String> {
        // Look for common icon file names near the game file
        let icon_names = vec!["icon.jpg", "icon.png", "cover.jpg", "cover.png"];
        
        if let Some(parent) = game_path.parent() {
            for icon_name in icon_names {
                let icon_path = parent.join(icon_name);
                if icon_path.exists() {
                    if let Ok(data) = fs::read(&icon_path) {
                        return Some(general_purpose::STANDARD.encode(&data));
                    }
                }
            }
        }
        
        None
    }

    /// Get all discovered games
    pub fn get_games(&self) -> Vec<Game> {
        self.games.clone()
    }

    /// Add a game manually
    pub fn add_game(&mut self, title: String, path: PathBuf, emulator: String) -> Result<Game, String> {
        if !path.exists() {
            return Err("Game file does not exist".to_string());
        }

        let id = format!("{:x}", md5::compute(path.to_string_lossy().as_bytes()));
        let icon = self.find_game_icon(&path);

        let game = Game {
            id,
            title,
            path,
            icon,
            emulator,
        };

        self.games.push(game.clone());
        Ok(game)
    }
}

/// Launch a game with the specified emulator
pub fn launch_game(game: &Game) -> Result<(), String> {
    use std::process::Command;

    let emulator_cmd = match game.emulator.as_str() {
        "yuzu" => {
            // Try to find yuzu executable
            if cfg!(target_os = "windows") {
                "yuzu.exe"
            } else if cfg!(target_os = "macos") {
                "yuzu"
            } else {
                "yuzu"
            }
        }
        "ryujinx" => {
            // Try to find Ryujinx executable
            if cfg!(target_os = "windows") {
                "Ryujinx.exe"
            } else if cfg!(target_os = "macos") {
                "Ryujinx"
            } else {
                "Ryujinx"
            }
        }
        _ => return Err("Unknown emulator".to_string()),
    };

    let result = Command::new(emulator_cmd)
        .arg(&game.path)
        .spawn();

    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to launch game: {}", e)),
    }
}
