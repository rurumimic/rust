//! Core 크레이트 - 설정 로딩 및 변환
//!
//! config-rs를 사용하여 YAML 파일을 읽고,
//! config-schema의 Raw 타입으로 역직렬화한 후,
//! fruits의 도메인 타입으로 변환합니다.

use config::{Config, File};
use config_schema::SettingsRaw;
use fruits::FruitConfig;

#[derive(Debug)]
pub struct AppConfig {
    pub app: String,
    pub version: String,
    pub fruit: FruitConfig,
}

impl AppConfig {
    /// 설정 파일 로드 및 변환
    pub fn load(config_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Self::load_with_warn(config_path, |msg| eprintln!("WARN: {}", msg))
    }

    /// 설정 파일 로드 (커스텀 warn 함수)
    pub fn load_with_warn<F>(
        config_path: &str,
        warn_fn: F,
    ) -> Result<Self, Box<dyn std::error::Error>>
    where
        F: Fn(&str) + Clone,
    {
        // 1. config-rs로 파일 로드
        let settings = Config::builder()
            .add_source(File::with_name(config_path))
            .build()?;

        // 2. Raw 스키마로 역직렬화
        let raw: SettingsRaw = settings.try_deserialize()?;

        // 3. 도메인 타입으로 변환 (검증 포함)
        let fruit = FruitConfig::try_from_raw(&raw.fruit, warn_fn)?;

        Ok(AppConfig {
            app: raw.app,
            version: raw.version,
            fruit,
        })
    }
}
