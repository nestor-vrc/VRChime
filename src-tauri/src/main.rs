#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows" // Disable terminal on Windows in release mode
)]

use serde::{Deserialize, Serialize}; // For serializing and deserializing structs to/from YAML.
use std::fs; // File system module for file operations.
use std::path::Path; // Path utilities for handling file paths.
use std::process::Command; // For executing external commands (launching VRChat).
use winreg::enums::*; // For registry access on Windows.
use winreg::RegKey; // Registry key handling.

// Struct to represent the configuration data.
#[derive(Serialize, Deserialize)]
struct Config {
    game_path: String, // The directory path where VRChat is installed.
}

// Function to save the game directory path in a YAML configuration file.
fn save_config(game_path: &str) -> Result<(), String> {
    let config = Config {
        game_path: game_path.to_string(), // Wrap the game directory in the Config struct.
    };

    // Determine the path to the temp directory and store the config in a subfolder "VRChime".
    let temp_dir = std::env::temp_dir();
    let config_path = temp_dir.join("VRChime/config.yaml");

    // Ensure the parent directories exist; create them if necessary.
    if let Some(parent) = config_path.parent() {
        if let Err(err) = fs::create_dir_all(parent) {
            return Err(format!("Error creating directories: {}", err)); // Return an error if directory creation fails.
        }
    }

    // Write the YAML string of the Config struct to the config file.
    match fs::write(&config_path, serde_yaml::to_string(&config).unwrap()) {
        Ok(_) => Ok(()), // If successful, return Ok.
        Err(err) => Err(format!("Error writing config file: {}", err)), // Return an error if file writing fails.
    }
}

// Function to load VRChat install path from the Windows registry.
fn load_registry_vrc_install_path() -> Option<String> {
    // Open the "Software\\VRChat" registry key from HKEY_CURRENT_USER.
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    if let Ok(vrchat_key) = hkcu.open_subkey("Software\\VRChat") {
        // Try to read the default value from the VRChat registry key.
        if let Ok(path) = vrchat_key.get_value::<String, _>("") {
            return Some(format!("{}\\VRChat.exe", path));
        }
    }
    None // Return None if the registry key or value isn't found.
}

// Tauri command to retrieve the configuration file's content.
#[tauri::command]
fn get_config() -> String {
    let temp_dir = std::env::temp_dir();
    let config_path = temp_dir.join("VRChime/config.yaml");

    // Attempt to read the config file; if found, return its content.
    if let Ok(config) = fs::read_to_string(&config_path) {
        return config;
    }

    // If the config file is missing or unreadable, fall back to the registry.
    if let Some(registry_path) = load_registry_vrc_install_path() {
        // If a registry path is found, return a JSON config with that value.
        return serde_yaml::to_string(&Config {
            game_path: registry_path,
        })
        .unwrap();
    }

    // If both the config file and registry are missing/unreadable, return a default empty config.
    r#"game_path: """#.to_string()
}

// Tauri command to get the version of the app.
#[tauri::command]
fn get_version() -> String {
    const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");
    VERSION.unwrap_or("Unknown").to_string()
}

// Tauri command to launch VRChat instances based on user input.
#[tauri::command]
fn launch_vrchat(game_path: String, vrcw_file: String, client_count: u32) -> String {
    // Basic validation: Ensure the game directory and VRCW file are provided.
    if game_path.is_empty() || vrcw_file.is_empty() {
        return "Error: Game path or file path is empty.".to_string();
    }

    // Construct the path to the VRChat executable.
    if !Path::new(&game_path).exists() {
        return format!("Error: VRChat.exe not found at {}", game_path); // Error if VRChat.exe is missing.
    }

    // Save the game directory path to the configuration file.
    if let Err(err) = save_config(&game_path) {
        return format!("Error saving config: {}", err); // Handle errors in saving the config.
    }

    // Loop through and spawn multiple instances of VRChat.
    for i in 0..client_count {
        // Construct command-line arguments to open a VRChat instance with the given VRCW file.
        let args = format!(
            "--url=create?hidden=true&name=BuildAndRun&url=file:///{}",
            vrcw_file
        );

        // Use Command to spawn a new VRChat process with the constructed arguments.
        if Command::new(&game_path)
            .args(args.split_whitespace()) // Split args string into space-separated arguments.
            .spawn()
            .is_err()
        {
            return format!("Error: Failed to launch VRChat instance {}", i + 1);
            // Error handling for launch failure.
        }
    }

    // Return a success message indicating the number of VRChat instances launched.
    format!("Successfully launched {} VRChat instance(s)!", client_count)
}

fn main() {
    // Set up the Tauri app and define the available commands (`get_config`, `get_version`, and `launch_vrchat`).
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_config,
            get_version,
            launch_vrchat
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
