use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::path::Path;

// Define a struct that matches your YAML structure
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    correct_caps_lock: bool,
    correct_suspicious_last_word: bool,
    display_warning: bool,
}

impl Config {
    pub fn default() -> Config {
        Config {
            correct_caps_lock: true,
            correct_suspicious_last_word: true,
            display_warning: true,
        }
    }
}

pub fn get_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_path = Path::new("./config.yaml");
    let mut file = File::open(config_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let config: Config = serde_yaml::from_str(&contents)?;
    Ok(config)
}
