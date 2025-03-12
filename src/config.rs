use serde::Deserialize;
use std::{fs, path::Path};

/// Default helpers
fn default_base_directory() -> String {
    "data".to_string()
}
fn default_dir_permissions() -> u32 {
    0o700
}
fn default_file_permissions() -> u32 {
    0o600
}
fn default_max_file_size_mb() -> u64 {
    2
}
fn default_device_path() -> String {
    "/dev/input/mice".to_string()
}

/// Defines how and where we store captured data.
#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct StorageConfig {
    #[serde(default = "default_base_directory")]
    pub base_directory: String,

    #[serde(default = "default_dir_permissions")]
    pub directory_permissions: u32,

    #[serde(default = "default_file_permissions")]
    pub file_permissions: u32,

    #[serde(default = "default_max_file_size_mb")]
    pub max_file_size_mb: u64,

    /// If true, we store 4 bytes per event ([button, dx, dy, 0]) instead of 3.
    #[serde(default)]
    pub store_4_bytes: bool,

    #[serde(default = "default_device_path")]
    pub device_path: String,
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            base_directory: default_base_directory(),
            directory_permissions: default_dir_permissions(),
            file_permissions: default_file_permissions(),
            max_file_size_mb: default_max_file_size_mb(),
            store_4_bytes: false,
            device_path: default_device_path(),
        }
    }
}

/// Top-level config (could have more sections in the future).
#[derive(Debug, Deserialize, Default)]
#[serde(default)]
pub struct Config {
    pub storage: StorageConfig,
}

impl Config {
    /// Loads from a TOML file, e.g. `mouse-entropy.toml`.
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let contents = fs::read_to_string(path)?;
        let cfg = toml::from_str::<Self>(&contents)?;
        Ok(cfg)
    }
}
