use std::{error::Error, fmt::Display, path::PathBuf};

use serde::{Deserialize, Serialize};

const CONFIG_NAME: &str = "ruaconf.toml";
const DEFAULT_NATIVE_ENTRY: &str = "native";
const DEFAULT_PLATFORM_ENTRY: &str = "lib";

#[derive(Debug, Clone, Default)]
pub struct RuaConfig {
    root_dir: String,
    data: RuaConfigData,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct RuaConfigData {
    native_entry: Option<String>,
    platform_entry: Option<String>,
}

#[derive(Debug)]
pub enum RuaConfigError {
    IoError(std::io::Error),
    TomlSerializeError(toml::de::Error),
    TomlDeserializeError(toml::ser::Error),
    NotFound,
}

impl Display for RuaConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuaConfigError::IoError(e) => write!(f, "IO error: {}", e),
            RuaConfigError::TomlSerializeError(e) => {
                write!(f, "TOMLSerializeError: {}", e)
            }
            RuaConfigError::TomlDeserializeError(e) => {
                write!(f, "TOMLDeserializeError: {}", e)
            }
            RuaConfigError::NotFound => write!(f, "Rua config not found"),
        }
    }
}

impl Error for RuaConfigError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            RuaConfigError::IoError(e) => Some(e),
            RuaConfigError::TomlSerializeError(e) => Some(e),
            RuaConfigError::TomlDeserializeError(e) => Some(e),
            RuaConfigError::NotFound => None,
        }
    }
}

impl RuaConfig {
    pub fn load() -> Result<Self, RuaConfigError> {
        let mut current_dir =
            std::env::current_dir().map_err(RuaConfigError::IoError)?;
        while !current_dir.join(CONFIG_NAME).exists() {
            let parent = current_dir.parent();
            if parent.is_none() {
                return Err(RuaConfigError::NotFound);
            }
            current_dir = parent.unwrap().to_path_buf();
        }
        let config_path = current_dir.join(CONFIG_NAME);
        let config_str = std::fs::read_to_string(config_path)
            .map_err(RuaConfigError::IoError)?;
        let config_data: RuaConfigData = toml::from_str(&config_str)
            .map_err(RuaConfigError::TomlSerializeError)?;
        Ok(RuaConfig {
            root_dir: current_dir.to_str().unwrap().to_string(),
            data: config_data,
        })
    }

    pub fn load_or_default() -> Self {
        match Self::load() {
            Ok(config) => config,
            Err(e) => {
                log::warn!("Failed to load Rua config, using default: {}", e);
                let root_dir = std::env::current_dir()
                    .expect("Failed to get current dir")
                    .to_str()
                    .expect("Failed to convert current dir to string")
                    .to_string();
                RuaConfig {
                    root_dir,
                    data: RuaConfigData {
                        native_entry: Some(DEFAULT_NATIVE_ENTRY.to_string()),
                        platform_entry: Some(
                            DEFAULT_PLATFORM_ENTRY.to_string(),
                        ),
                    },
                }
            }
        }
    }

    pub fn save(&self) -> Result<(), RuaConfigError> {
        let config_path: PathBuf = self.root_dir.clone().into();
        let config_path = config_path.join(CONFIG_NAME);
        let config_str = toml::to_string(&self.data)
            .map_err(RuaConfigError::TomlDeserializeError)?;
        std::fs::write(config_path, config_str)
            .map_err(RuaConfigError::IoError)?;
        Ok(())
    }

    pub fn get_native_entry(&self) -> &str {
        self.data
            .native_entry
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or(DEFAULT_NATIVE_ENTRY)
    }

    pub fn get_platform_entry(&self) -> &str {
        self.data
            .platform_entry
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or(DEFAULT_PLATFORM_ENTRY)
    }

    pub fn set_native_entry(&mut self, entry: &str) {
        self.data.native_entry = Some(entry.to_string());
    }

    pub fn set_platform_entry(&mut self, entry: &str) {
        self.data.platform_entry = Some(entry.to_string());
    }
}
