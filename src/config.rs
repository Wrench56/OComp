use serde::Deserialize;
use std::collections::HashMap;
use std::{fs, path::PathBuf};
use toml;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub version: String,
    #[serde(flatten)]
    pub sections: HashMap<String, BuildConfig>,
}

#[derive(Debug, Deserialize)]
pub struct BuildConfig {
    pub build: String,
    pub output: String,
}

pub fn load_config() -> Config {
    let config_path = ensure_config_exists();
    let content = fs::read_to_string(config_path).expect("Failed to read config file");
    toml::from_str(&content).expect("Invalid TOML configuration")
}

fn get_config_dir() -> PathBuf {
    if cfg!(target_os = "windows") {
        PathBuf::from(std::env::var("APPDATA").unwrap_or_else(|_| "".to_string())).join("ocomp")
    } else {
        PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| "".to_string()))
            .join(".config")
            .join("ocomp")
    }
}

fn ensure_config_exists() -> PathBuf {
    let config_dir = get_config_dir();
    let config_path = config_dir.join("config.toml");

    if !config_path.exists() {
        fs::create_dir_all(&config_dir).expect("Failed to create config directory");
        let default_config = r#"version = "1.0.0"

[rust]
    build = "cargo build --release"
    output = "target/release/*.exe"
"#;
        fs::write(&config_path, default_config).expect("Failed to write default config");
    }

    config_path
}
