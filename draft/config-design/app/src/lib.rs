//! Core crate for config loading and conversion.
//!
//! Reads YAML via config-rs, deserializes into raw schema types,
//! then converts into domain types with validation.

use config::{Config, File};
use fruits::FruitConfig;
use schema::{
    HealthSettingsRaw, LogFormat, LogLevel, LogOutput, LoggerSettingsRaw, RedisSettingsRaw,
    SettingsRaw,
};
mod error;

pub use error::AppConfigError;
pub use error::ValidationError;
pub use schema::SchemaError;

#[derive(Debug)]
pub struct AppConfig {
    pub app: String,
    pub version: String,
    pub fruit: FruitConfig,
    pub logger: LoggerConfig,
    pub health: HealthConfig,
    pub redis: RedisConfig,
}

#[derive(Debug, Clone)]
pub struct LoggerConfig {
    pub level: LogLevel,
    pub format: LogFormat,
    pub output: LogOutput,
    pub file: Option<String>,
}

impl TryFrom<LoggerSettingsRaw> for LoggerConfig {
    type Error = ValidationError;

    fn try_from(raw: LoggerSettingsRaw) -> Result<Self, Self::Error> {
        let config = LoggerConfig {
            level: raw.level,
            format: raw.format,
            output: raw.output,
            file: raw.file,
        };

        config.validate()?;

        Ok(config)
    }
}

impl LoggerConfig {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.output == LogOutput::File {
            match self.file.as_deref() {
                Some(path) if !path.trim().is_empty() => Ok(()),
                _ => Err(ValidationError::LoggerFileRequired),
            }
        } else {
            Ok(())
        }
    }
}

#[derive(Debug, Clone)]
pub struct HealthConfig {
    pub enabled: bool,
    pub path: String,
    pub timeout_ms: u64,
}

impl TryFrom<HealthSettingsRaw> for HealthConfig {
    type Error = ValidationError;

    fn try_from(raw: HealthSettingsRaw) -> Result<Self, Self::Error> {
        let config = HealthConfig {
            enabled: raw.enabled,
            path: raw.path,
            timeout_ms: raw.timeout_ms,
        };

        config.validate()?;

        Ok(config)
    }
}

impl HealthConfig {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.path.trim().is_empty() {
            return Err(ValidationError::HealthPathEmpty);
        }

        if !self.path.starts_with('/') {
            return Err(ValidationError::HealthPathNotAbsolute);
        }

        if self.timeout_ms == 0 {
            return Err(ValidationError::HealthTimeoutZero);
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct RedisConfig {
    pub url: String,
    pub pool_size: u32,
    pub connect_timeout_ms: u64,
}

impl TryFrom<RedisSettingsRaw> for RedisConfig {
    type Error = ValidationError;

    fn try_from(raw: RedisSettingsRaw) -> Result<Self, Self::Error> {
        let config = RedisConfig {
            url: raw.url,
            pool_size: raw.pool_size,
            connect_timeout_ms: raw.connect_timeout_ms,
        };

        config.validate()?;

        Ok(config)
    }
}

impl RedisConfig {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.url.trim().is_empty() {
            return Err(ValidationError::RedisUrlEmpty);
        }

        if self.pool_size == 0 {
            return Err(ValidationError::RedisPoolSizeZero);
        }

        if self.connect_timeout_ms == 0 {
            return Err(ValidationError::RedisConnectTimeoutZero);
        }

        Ok(())
    }
}

impl TryFrom<SettingsRaw> for AppConfig {
    type Error = AppConfigError;

    fn try_from(raw: SettingsRaw) -> Result<Self, Self::Error> {
        let logger: LoggerConfig = raw.logger.try_into()?;
        let health: HealthConfig = raw.health.try_into()?;
        let redis: RedisConfig = raw.redis.try_into()?;
        let fruit: FruitConfig = raw.fruit.try_into()?;

        Ok(AppConfig {
            app: raw.app,
            version: raw.version,
            fruit,
            logger,
            health,
            redis,
        })
    }
}

impl AppConfig {
    /// Load configuration from a YAML file.
    #[must_use = "this returns the loaded config, which should be used"]
    pub fn load(config_path: &str) -> Result<Self, AppConfigError> {
        let settings = Config::builder()
            .add_source(File::with_name(config_path))
            .build()?;

        let raw: SettingsRaw = settings.try_deserialize()?;

        raw.try_into()
    }
}
