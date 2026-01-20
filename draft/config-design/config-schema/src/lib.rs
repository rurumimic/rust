//! 설정 입력 스키마 정의
//!
//! 이 크레이트는 YAML/JSON 등에서 읽어온 "Raw" 설정을 정의합니다.
//! 경계(boundary)에서만 serde_json::Value를 사용하고,
//! 도메인 크레이트에서 타입 안전한 구조체로 변환합니다.

pub mod error;
pub mod fruit;
pub mod health;
pub mod logger;
pub mod policy;
pub mod redis;
pub mod settings;

pub use error::SchemaError;
pub use fruit::{
    AppleOptions, AppleSettingsRaw, BananaOptions, BananaSettingsRaw, Curvature, FruitSettingsRaw,
    OrangeOptions, OrangeSettingsRaw,
};
pub use health::HealthSettingsRaw;
pub use logger::{LogFormat, LogLevel, LogOutput, LoggerSettingsRaw};
pub use policy::UnknownKeyPolicy;
pub use redis::RedisSettingsRaw;
pub use settings::SettingsRaw;
