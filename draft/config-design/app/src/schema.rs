//! Input schema definitions.
//!
//! Defines "raw" settings read from YAML/JSON and converts them into
//! type-safe domain structures at the boundary.

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
