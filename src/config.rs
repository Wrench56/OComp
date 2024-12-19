use semver::Version;
use serde::Deserialize;
use std::collections::HashMap;
use std::{fs, io, path::PathBuf};
use toml;

const CURRENT_CONFIG_VERSION: Version = Version::new(1, 0, 0);

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
    let content = fs::read_to_string(&config_path).expect("Failed to read config file");
    let config: Config = toml::from_str(&content).expect("Invalid TOML configuration");

    let config_version = Version::parse(&config.version).expect("Invalid config version");
    if config_version.major != CURRENT_CONFIG_VERSION.major
        || config_version.minor != CURRENT_CONFIG_VERSION.minor
    {
        println!(
            "Config version mismatch: executable's config version {} does not match current config version {}.\nWould you like to overwrite the config with the newer default version?\nNote that such a change removes all your current configurations and replaces it with the default configuration.\nNot updating your configuration to the latest verion will cause ocomp to crash. [y/N]",
            CURRENT_CONFIG_VERSION, config_version
        );

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        if input.trim().eq_ignore_ascii_case("y") {
            fs::remove_file(&config_path).expect("Failed to delete old config file");
            return load_config();
        } else {
            eprintln!("Exiting without changes.");
            std::process::exit(1);
        }
    }

    config
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
        let default_config = format!(
            r#"version = "{CURRENT_CONFIG_VERSION}"

[rust]
    build = "cargo build --release"
    output = "target/release/*.exe"
"#
        );
        fs::write(&config_path, default_config).expect("Failed to write default config");
    }

    config_path
}
