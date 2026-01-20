use serde::Deserialize;

use crate::{FruitSettingsRaw, HealthSettingsRaw, LoggerSettingsRaw, RedisSettingsRaw};

/// 전체 설정 파일의 Raw 스키마
#[derive(Debug, Clone, Deserialize)]
pub struct SettingsRaw {
    /// 앱 이름
    pub app: String,

    /// 버전
    pub version: String,

    /// Logger 설정
    #[serde(default)]
    pub logger: LoggerSettingsRaw,

    /// Health 설정
    #[serde(default)]
    pub health: HealthSettingsRaw,

    /// Redis 설정
    #[serde(default)]
    pub redis: RedisSettingsRaw,

    /// Fruit 설정
    pub fruit: FruitSettingsRaw,
}
