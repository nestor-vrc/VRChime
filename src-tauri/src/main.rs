#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows" // Disable terminal on Windows in release mode
)]

use tauri::generate_context;
use tauri::Builder;

mod commands; // Import commands module
mod config; // Import config module
mod error; // Import error module

fn main() {
    // Initialize logging
    env_logger::init();

    // Set up the Tauri app and define the available commands.
    Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::get_config,
            commands::get_version,
            commands::launch_vrchat
        ])
        .run(generate_context!())
        .expect("error while running tauri application");
}
