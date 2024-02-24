use dirs;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{Read, Write};

// Define a struct that matches your YAML structure
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub correct_caps_lock: bool,
    pub correct_suspicious_last_word: bool,
    pub display_warning: bool,
    pub path: String,
}

impl Config {
    pub fn default() -> Config {
        Config {
            correct_caps_lock: true,
            correct_suspicious_last_word: true,
            display_warning: true,
            path: String::new()
        }
    }
    pub fn default_with_path(path: String) -> Config {
        Config {
            correct_caps_lock: true,
            correct_suspicious_last_word: true,
            display_warning: true,
            path: path
        }
    }
}

pub fn get_config() -> Result<Config, Box<dyn std::error::Error>> {
    if let Some(mut config_dir) = dirs::config_dir() {
        config_dir.push("capslock-auto-switch");
        let _ = fs::create_dir_all(config_dir.clone());
        config_dir.push("config.yml");
        println!("Config directory should be: {:?}", config_dir); 

        let file = File::open(config_dir.clone());
        match file {
            Ok(mut f) => {
                let mut contents = String::new();
                f.read_to_string(&mut contents)?;
                let config: Config = serde_yaml::from_str(&contents)?;
                Ok(config)
            }
            Err(_) => {
                println!("Creating default config file {}", config_dir.clone().to_str().unwrap());
                let config = Config::default_with_path(config_dir.to_str().unwrap().to_string());
                let ff = File::create(config_dir.clone());
                match ff {
                    Ok(mut file) => {
                        println!("Default config file created");
                        let contents = serde_yaml::to_string(&config)?;
                        file.write_all(contents.as_bytes())?;
                        Ok(config)
                    }
                    Err( err ) => {
                        println!("Unable to create config file {:?}", err);
                        Ok(Config::default())
                    }
                    
                } 
            }
        }
    } else {
        println!("Unable to find config directory");
        Ok(Config::default())
    }
}
