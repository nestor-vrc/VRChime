#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::path::Path;
use std::process::Command;

#[tauri::command]
fn launch_vrchat(game_dir: String, vrcw_file: String, client_count: u32) -> String {
    if game_dir.is_empty() || vrcw_file.is_empty() {
        return "Error: Game directory or file path is empty.".to_string();
    }

    for i in 0..client_count {
        let exe_path = format!("{}/VRChat.exe", game_dir);

        if !Path::new(&exe_path).exists() {
            return format!("Error: VRChat.exe not found at {}", game_dir);
        }

        let args = format!(
            "--url=create?hidden=true&name=BuildAndRun&url=file:///{}",
            vrcw_file
        );

        if Command::new(&exe_path)
            .args(args.split_whitespace())
            .spawn()
            .is_err()
        {
            return format!("Error: Failed to launch VRChat instance {}", i + 1);
        }
    }

    format!("Successfully launched {} VRChat instance(s)!", client_count)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![launch_vrchat])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
