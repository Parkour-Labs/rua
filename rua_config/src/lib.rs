use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RuaConfig {
    native_entry: Option<String>,
    platform_entry: Option<String>,
}
