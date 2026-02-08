use std::sync::Mutex;
use tauri::State;

mod games;
use games::{Game, GameScanner, launch_game};

struct AppState {
    scanner: Mutex<GameScanner>,
}

#[tauri::command]
fn scan_games(state: State<AppState>) -> Result<Vec<Game>, String> {
    let mut scanner = state.scanner.lock().map_err(|e| e.to_string())?;
    
    // Scan both emulators
    scanner.scan_yuzu()?;
    scanner.scan_ryujinx()?;
    
    Ok(scanner.get_games())
}

#[tauri::command]
fn get_games(state: State<AppState>) -> Result<Vec<Game>, String> {
    let scanner = state.scanner.lock().map_err(|e| e.to_string())?;
    Ok(scanner.get_games())
}

#[tauri::command]
fn add_game(
    state: State<AppState>,
    title: String,
    path: String,
    emulator: String,
) -> Result<Game, String> {
    let mut scanner = state.scanner.lock().map_err(|e| e.to_string())?;
    scanner.add_game(title, std::path::PathBuf::from(path), emulator)
}

#[tauri::command]
fn launch_game_cmd(game: Game) -> Result<(), String> {
    launch_game(&game)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
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
    .manage(AppState {
        scanner: Mutex::new(GameScanner::new()),
    })
    .invoke_handler(tauri::generate_handler![
        scan_games,
        get_games,
        add_game,
        launch_game_cmd
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
