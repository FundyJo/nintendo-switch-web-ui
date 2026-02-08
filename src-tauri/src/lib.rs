mod games;

use games::{scan_all_games, scan_custom_games, Game};

#[tauri::command]
fn get_games() -> Vec<Game> {
    scan_all_games()
}

#[tauri::command]
fn scan_directory(directory: String) -> Vec<Game> {
    scan_custom_games(directory)
}

#[tauri::command]
async fn launch_game(emulator_path: String, rom_path: String) -> Result<(), String> {
    use std::process::Command;
    
    Command::new(emulator_path)
        .arg(rom_path)
        .spawn()
        .map_err(|e| e.to_string())?;
    
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_fs::init())
    .plugin(tauri_plugin_dialog::init())
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![get_games, scan_directory, launch_game])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
