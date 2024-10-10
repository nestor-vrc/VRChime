use crate::error::AppError;
use serde::{Deserialize, Serialize};
use std::fs;
use winreg::enums::*;
use winreg::RegKey;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub game_path: String, // The directory path where VRChat is installed.
}

// Function to save the game directory path in a YAML configuration file.
pub fn save_config(game_path: &str) -> Result<(), AppError> {
    let config = Config {
        game_path: game_path.to_string(), // Wrap the game directory in the Config struct.
    };

    // Determine the path to the temp directory and store the config in a subfolder "VRChime".
    let temp_dir = std::env::temp_dir();
    let config_path = temp_dir.join("VRChime/config.yaml");

    // Ensure the parent directories exist; create them if necessary.
    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent)?;
    }

    // Write the YAML string of the Config struct to the config file.
    fs::write(&config_path, serde_yaml::to_string(&config)?)?;
    Ok(()) // If successful, return Ok.
}

// Function to load VRChat install path from the Windows registry.
pub fn load_registry_vrc_install_path() -> Result<String, AppError> {
    // Open the "Software\\VRChat" registry key from HKEY_CURRENT_USER.
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let vrchat_key = hkcu
        .open_subkey("Software\\VRChat")
        .map_err(|_| AppError::RegistryError("Failed to open registry key".to_string()))?;

    // Try to read the default value from the VRChat registry key.
    let path: String = vrchat_key
        .get_value("") // Default value
        .map_err(|_| AppError::RegistryError("Failed to read registry value".to_string()))?;

    Ok(format!("{}\\VRChat.exe", path))
}
