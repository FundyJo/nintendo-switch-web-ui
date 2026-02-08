use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use base64::{Engine as _, engine::general_purpose};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Game {
    pub id: String,
    pub title: String,
    pub path: String, // Changed to String for serialization
    pub icon: Option<String>, // Base64 encoded icon or URL
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
                // Additional common game storage locations
                home_dir.join("Documents/Yuzu/games"),
                home_dir.join("Games/Switch"),
                home_dir.join("Games/Yuzu"),
                home_dir.join("Downloads/Switch"),
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
            // Typical Ryujinx configuration directory locations
            let ryujinx_config_dirs = vec![
                home_dir.join(".config/Ryujinx"),
                home_dir.join("AppData/Roaming/Ryujinx"),
                home_dir.join("Library/Application Support/Ryujinx"),
            ];

            // First, try to find and parse Ryujinx's application database
            for config_dir in &ryujinx_config_dirs {
                if config_dir.exists() {
                    let _ = self.scan_ryujinx_database(&config_dir);
                    
                    // Also scan common game directories
                    let games_dir = config_dir.join("games");
                    if games_dir.exists() {
                        self.scan_directory(&games_dir, "ryujinx")?;
                    }
                }
            }

            // Also check for portable Ryujinx installations
            let portable_paths = vec![
                PathBuf::from("C:/Ryujinx/games"),
                home_dir.join("Ryujinx/games"),
            ];

            for portable_path in portable_paths {
                if portable_path.exists() {
                    self.scan_directory(&portable_path, "ryujinx")?;
                }
            }
        }
        Ok(())
    }

    /// Scan Ryujinx's application database for games with icons
    fn scan_ryujinx_database(&mut self, config_dir: &Path) -> Result<(), String> {
        // Ryujinx stores application info in a SQLite database or JSON files
        // The games.json or title.json file may contain game information
        let games_json = config_dir.join("games.json");
        let cache_dir = config_dir.join("bis/user/save/0000000000000000/0000000000000000/cache");
        
        // Try to find icon cache in Ryujinx's cache directory
        if cache_dir.exists() {
            for entry in WalkDir::new(&cache_dir)
                .max_depth(2)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                if let Some(file_name) = entry.file_name().to_str() {
                    if file_name.ends_with(".jpg") || file_name.ends_with(".png") {
                        // This might be a cached game icon
                        // The file name should contain the title ID
                        if let Some(parent) = entry.path().parent() {
                            if let Some(title_id) = parent.file_name() {
                                let title_id_str = title_id.to_string_lossy().to_string();
                                // Only process if it looks like a title ID (16 hex chars)
                                if title_id_str.len() == 16 && title_id_str.chars().all(|c| c.is_ascii_hexdigit()) {
                                    if let Ok(icon_data) = fs::read(entry.path()) {
                                        let icon_base64 = general_purpose::STANDARD.encode(&icon_data);
                                        // Try to find the corresponding game file
                                        // For now, we'll just note that we found an icon for this title ID
                                        log::info!("Found icon for title ID: {}", title_id_str);
                                    }
                                }
                            }
                        }
                    }
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
                        
                        // Try multiple icon strategies
                        let icon = self.find_game_icon(path)
                            .or_else(|| self.extract_title_id_and_fetch_icon(path))
                            .or_else(|| self.get_default_icon());
                        
                        self.games.push(Game {
                            id,
                            title,
                            path: path.to_string_lossy().to_string(),
                            icon,
                            emulator: emulator.to_string(),
                        });
                    }
                }
            }
        }
        Ok(())
    }

    /// Try to extract title ID from filename and fetch icon from online source
    fn extract_title_id_and_fetch_icon(&self, game_path: &Path) -> Option<String> {
        // Try to extract title ID from filename
        // Nintendo Switch title IDs are 16 hex characters
        if let Some(file_name) = game_path.file_name() {
            let name_str = file_name.to_string_lossy();
            // Look for patterns like [0100000000010000] or 0100000000010000
            for word in name_str.split(|c: char| !c.is_ascii_alphanumeric()) {
                if word.len() == 16 && word.chars().all(|c| c.is_ascii_hexdigit()) {
                    // Found a potential title ID
                    // Return URL to tinfoil.media icon
                    return Some(format!("https://tinfoil.media/ti/{}/512/512", word.to_uppercase()));
                }
            }
        }
        None
    }

    /// Get a default placeholder icon
    fn get_default_icon(&self) -> Option<String> {
        Some("https://via.placeholder.com/512x512/151515/FFFFFF?text=No+Icon".to_string())
    }

    /// Try to find an icon for the game
    fn find_game_icon(&self, game_path: &Path) -> Option<String> {
        // First, look for common icon file names near the game file
        let icon_names = vec!["icon.jpg", "icon.png", "cover.jpg", "cover.png", "boxart.jpg", "boxart.png"];
        
        if let Some(parent) = game_path.parent() {
            for icon_name in icon_names {
                let icon_path = parent.join(icon_name);
                if icon_path.exists() {
                    if let Ok(data) = fs::read(&icon_path) {
                        // Return as base64 data URL
                        let mime_type = if icon_name.ends_with(".png") { "image/png" } else { "image/jpeg" };
                        return Some(format!("data:{};base64,{}", mime_type, general_purpose::STANDARD.encode(&data)));
                    }
                }
            }
            
            // Also check if there's a folder with the same name as the game containing icons
            if let Some(game_stem) = game_path.file_stem() {
                let game_folder = parent.join(game_stem);
                if game_folder.exists() && game_folder.is_dir() {
                    for icon_name in &["icon.jpg", "icon.png", "cover.jpg", "cover.png"] {
                        let icon_path = game_folder.join(icon_name);
                        if icon_path.exists() {
                            if let Ok(data) = fs::read(&icon_path) {
                                let mime_type = if icon_name.ends_with(".png") { "image/png" } else { "image/jpeg" };
                                return Some(format!("data:{};base64,{}", mime_type, general_purpose::STANDARD.encode(&data)));
                            }
                        }
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
        let icon = self.find_game_icon(&path)
            .or_else(|| self.extract_title_id_and_fetch_icon(&path))
            .or_else(|| self.get_default_icon());

        let game = Game {
            id,
            title,
            path: path.to_string_lossy().to_string(),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_title_id_from_filename() {
        let scanner = GameScanner::new();
        
        // Test with brackets
        let path1 = Path::new("/games/[0100000000010000] Super Mario Odyssey.nsp");
        let icon1 = scanner.extract_title_id_and_fetch_icon(path1);
        assert!(icon1.is_some());
        assert_eq!(icon1.unwrap(), "https://tinfoil.media/ti/0100000000010000/512/512");
        
        // Test without brackets
        let path2 = Path::new("/games/01007EF00011E000 - Zelda BOTW.xci");
        let icon2 = scanner.extract_title_id_and_fetch_icon(path2);
        assert!(icon2.is_some());
        assert_eq!(icon2.unwrap(), "https://tinfoil.media/ti/01007EF00011E000/512/512");
        
        // Test with no title ID
        let path3 = Path::new("/games/Some Game Without ID.nsp");
        let icon3 = scanner.extract_title_id_and_fetch_icon(path3);
        assert!(icon3.is_none());
    }

    #[test]
    fn test_default_icon() {
        let scanner = GameScanner::new();
        let icon = scanner.get_default_icon();
        assert!(icon.is_some());
        assert!(icon.unwrap().contains("placeholder"));
    }

    #[test]
    fn test_game_creation() {
        let game = Game {
            id: "test123".to_string(),
            title: "Test Game".to_string(),
            path: "/test/game.nsp".to_string(),
            icon: Some("https://example.com/icon.png".to_string()),
            emulator: "ryujinx".to_string(),
        };
        
        assert_eq!(game.title, "Test Game");
        assert_eq!(game.emulator, "ryujinx");
    }
}
