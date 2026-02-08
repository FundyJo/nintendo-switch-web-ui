use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Game {
    pub title: String,
    pub icon_path: String,
    pub rom_path: String,
    pub title_id: Option<String>,
}

/// Get the default Yuzu games directory based on the operating system
fn get_yuzu_directory() -> Option<PathBuf> {
    let home_dir = dirs::home_dir()?;
    
    #[cfg(target_os = "windows")]
    let yuzu_path = home_dir.join("AppData").join("Roaming").join("yuzu");
    
    #[cfg(target_os = "linux")]
    let yuzu_path = home_dir.join(".local").join("share").join("yuzu");
    
    #[cfg(target_os = "macos")]
    let yuzu_path = home_dir.join("Library").join("Application Support").join("yuzu");
    
    if yuzu_path.exists() {
        Some(yuzu_path)
    } else {
        None
    }
}

/// Get the default Ryujinx games directory based on the operating system
fn get_ryujinx_directory() -> Option<PathBuf> {
    let home_dir = dirs::home_dir()?;
    
    #[cfg(target_os = "windows")]
    let ryujinx_path = home_dir.join("AppData").join("Roaming").join("Ryujinx");
    
    #[cfg(target_os = "linux")]
    let ryujinx_path = home_dir.join(".config").join("Ryujinx");
    
    #[cfg(target_os = "macos")]
    let ryujinx_path = home_dir.join("Library").join("Application Support").join("Ryujinx");
    
    if ryujinx_path.exists() {
        Some(ryujinx_path)
    } else {
        None
    }
}

/// Scan for games in Yuzu's NAND directory
fn scan_yuzu_games(yuzu_dir: &Path) -> Vec<Game> {
    let mut games = Vec::new();
    
    // Yuzu stores game data in nand/user/Contents/registered
    let registered_path = yuzu_dir.join("nand").join("user").join("Contents").join("registered");
    
    if !registered_path.exists() {
        return games;
    }
    
    // Walk through the registered directory
    for entry in WalkDir::new(&registered_path)
        .max_depth(2)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        
        // Look for .nca files (Nintendo Content Archives)
        if path.extension().and_then(|s| s.to_str()) == Some("nca") {
            if let Some(parent) = path.parent() {
                if let Some(title_id) = parent.file_name().and_then(|s| s.to_str()) {
                    // Check for icon in the same directory
                    let icon_path = parent.join("icon_AmericanEnglish.dat");
                    if !icon_path.exists() {
                        continue;
                    }
                    
                    games.push(Game {
                        title: format!("Game {}", &title_id[..8]),
                        icon_path: icon_path.to_string_lossy().to_string(),
                        rom_path: path.to_string_lossy().to_string(),
                        title_id: Some(title_id.to_string()),
                    });
                }
            }
        }
    }
    
    games
}

/// Scan for games in Ryujinx's game directory
fn scan_ryujinx_games(ryujinx_dir: &Path) -> Vec<Game> {
    let mut games = Vec::new();
    
    // Ryujinx stores games in bis/user/Contents/registered
    let registered_path = ryujinx_dir.join("bis").join("user").join("Contents").join("registered");
    
    if !registered_path.exists() {
        return games;
    }
    
    // Walk through the registered directory
    for entry in WalkDir::new(&registered_path)
        .max_depth(2)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        
        // Look for .nca files
        if path.extension().and_then(|s| s.to_str()) == Some("nca") {
            if let Some(parent) = path.parent() {
                if let Some(title_id) = parent.file_name().and_then(|s| s.to_str()) {
                    // Check for icon
                    let icon_path = parent.join("icon_AmericanEnglish.dat");
                    if !icon_path.exists() {
                        continue;
                    }
                    
                    games.push(Game {
                        title: format!("Game {}", &title_id[..8]),
                        icon_path: icon_path.to_string_lossy().to_string(),
                        rom_path: path.to_string_lossy().to_string(),
                        title_id: Some(title_id.to_string()),
                    });
                }
            }
        }
    }
    
    games
}

/// Scan a custom directory for NSP/XCI files
fn scan_custom_directory(dir_path: &Path) -> Vec<Game> {
    let mut games = Vec::new();
    
    if !dir_path.exists() {
        return games;
    }
    
    for entry in WalkDir::new(dir_path)
        .max_depth(3)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        
        // Look for NSP or XCI files (common Switch ROM formats)
        if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
            if ext.eq_ignore_ascii_case("nsp") || ext.eq_ignore_ascii_case("xci") {
                if let Some(file_stem) = path.file_stem().and_then(|s| s.to_str()) {
                    games.push(Game {
                        title: file_stem.to_string(),
                        icon_path: String::new(), // Will need to extract from ROM
                        rom_path: path.to_string_lossy().to_string(),
                        title_id: None,
                    });
                }
            }
        }
    }
    
    games
}

/// Scan all available locations for games
pub fn scan_all_games() -> Vec<Game> {
    let mut all_games = Vec::new();
    
    // Scan Yuzu
    if let Some(yuzu_dir) = get_yuzu_directory() {
        let yuzu_games = scan_yuzu_games(&yuzu_dir);
        all_games.extend(yuzu_games);
    }
    
    // Scan Ryujinx
    if let Some(ryujinx_dir) = get_ryujinx_directory() {
        let ryujinx_games = scan_ryujinx_games(&ryujinx_dir);
        all_games.extend(ryujinx_games);
    }
    
    all_games
}

/// Scan a specific custom directory
pub fn scan_custom_games(directory: String) -> Vec<Game> {
    let path = PathBuf::from(directory);
    scan_custom_directory(&path)
}
