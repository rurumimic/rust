use serde::Deserialize;

use crate::{FruitSettingsRaw, HealthSettingsRaw, LoggerSettingsRaw, RedisSettingsRaw};

/// Raw schema for the full settings file.
#[derive(Debug, Clone, Deserialize)]
pub struct SettingsRaw {
    pub app: String,

    pub version: String,

    #[serde(default)]
    pub logger: LoggerSettingsRaw,

    #[serde(default)]
    pub health: HealthSettingsRaw,

    #[serde(default)]
    pub redis: RedisSettingsRaw,

    pub fruit: FruitSettingsRaw,
}
