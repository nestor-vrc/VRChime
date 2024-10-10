use tauri::InvokeError;
use thiserror::Error; // For custom error handling

#[derive(Error, Debug)]
pub enum AppError {
    #[error("File error: {0}")]
    FileError(#[from] std::io::Error),
    #[error("YAML error: {0}")]
    YamlError(#[from] serde_yaml::Error),
    #[error("Registry error: {0}")]
    RegistryError(String),
}

// Implement Into<InvokeError> for AppError
impl From<AppError> for InvokeError {
    fn from(err: AppError) -> Self {
        InvokeError::from(err.to_string()) // Convert AppError to a string
    }
}
