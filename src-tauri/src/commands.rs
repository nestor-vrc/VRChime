use crate::config::{load_registry_vrc_install_path, save_config, Config};
use crate::error::AppError;
use std::fs;
use std::path::Path;
use std::process::Command;

// Tauri command to retrieve the configuration file's content.
#[tauri::command]
pub fn get_config() -> Result<String, AppError> {
    let temp_dir = std::env::temp_dir();
    let config_path = temp_dir.join("VRChime/config.yaml");

    // Attempt to read the config file; if found, return its content.
    if let Ok(config) = fs::read_to_string(&config_path) {
        return Ok(config);
    }

    // If the config file is missing or unreadable, fall back to the registry.
    if let Ok(registry_path) = load_registry_vrc_install_path() {
        // If a registry path is found, return a JSON config with that value.
        return Ok(serde_yaml::to_string(&Config {
            game_path: registry_path,
        })?);
    }

    // If both the config file and registry are missing/unreadable, return a default empty config.
    Ok(r#"game_path: """#.to_string())
}

// Tauri command to get the version of the app.
#[tauri::command]
pub fn get_version() -> String {
    const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");
    VERSION.unwrap_or("Unknown").to_string()
}

// Tauri command to launch VRChat instances based on user input.
#[tauri::command]
pub fn launch_vrchat(
    game_path: String,
    vrcw_file: String,
    client_count: u32,
) -> Result<String, AppError> {
    // Basic validation: Ensure the game directory and VRCW file are provided.
    if game_path.is_empty() || vrcw_file.is_empty() {
        return Err(AppError::RegistryError(
            "Error: Game path or file path is empty.".to_string(),
        ));
    }

    // Construct the path to the VRChat executable.
    if !Path::new(&game_path).exists() {
        return Err(AppError::RegistryError(format!(
            "Error: VRChat.exe not found at {}",
            game_path
        )));
    }

    // Save the game directory path to the configuration file.
    save_config(&game_path)?;

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
            return Err(AppError::RegistryError(format!(
                "Error: Failed to launch VRChat instance {}",
                i + 1
            )));
        }
    }

    // Return a success message indicating the number of VRChat instances launched.
    Ok(format!(
        "Successfully launched {} VRChat instance(s)!",
        client_count
    ))
}
