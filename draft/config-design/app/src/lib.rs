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
use std::io;

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

impl TryFrom<&LoggerSettingsRaw> for LoggerConfig {
    type Error = io::Error;

    fn try_from(raw: &LoggerSettingsRaw) -> Result<Self, Self::Error> {
        let config = LoggerConfig {
            level: raw.level,
            format: raw.format,
            output: raw.output,
            file: raw.file.clone(),
        };

        config.validate()?;

        Ok(config)
    }
}

impl LoggerConfig {
    pub fn validate(&self) -> Result<(), io::Error> {
        if self.output == LogOutput::File {
            match self.file.as_deref() {
                Some(path) if !path.trim().is_empty() => Ok(()),
                _ => Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "logger.output=file requires a non-empty logger.file",
                )),
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

impl TryFrom<&HealthSettingsRaw> for HealthConfig {
    type Error = io::Error;

    fn try_from(raw: &HealthSettingsRaw) -> Result<Self, Self::Error> {
        let config = HealthConfig {
            enabled: raw.enabled,
            path: raw.path.clone(),
            timeout_ms: raw.timeout_ms,
        };

        config.validate()?;

        Ok(config)
    }
}

impl HealthConfig {
    pub fn validate(&self) -> Result<(), io::Error> {
        if self.path.trim().is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "health.path must be non-empty",
            ));
        }

        if !self.path.starts_with('/') {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "health.path must start with '/'",
            ));
        }

        if self.timeout_ms == 0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "health.timeout_ms must be greater than 0",
            ));
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

impl TryFrom<&RedisSettingsRaw> for RedisConfig {
    type Error = io::Error;

    fn try_from(raw: &RedisSettingsRaw) -> Result<Self, Self::Error> {
        let config = RedisConfig {
            url: raw.url.clone(),
            pool_size: raw.pool_size,
            connect_timeout_ms: raw.connect_timeout_ms,
        };

        config.validate()?;

        Ok(config)
    }
}

impl RedisConfig {
    pub fn validate(&self) -> Result<(), io::Error> {
        if self.url.trim().is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "redis.url must be non-empty",
            ));
        }

        if self.pool_size == 0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "redis.pool_size must be greater than 0",
            ));
        }

        if self.connect_timeout_ms == 0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "redis.connect_timeout_ms must be greater than 0",
            ));
        }

        Ok(())
    }
}

impl AppConfig {
    pub fn load(config_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let settings = Config::builder()
            .add_source(File::with_name(config_path))
            .build()?;

        let raw: SettingsRaw = settings.try_deserialize()?;

        AppConfig::try_from_raw(raw)
    }

    pub fn try_from_raw(raw: SettingsRaw) -> Result<Self, Box<dyn std::error::Error>> {
        let logger = LoggerConfig::try_from(&raw.logger)?;
        let health = HealthConfig::try_from(&raw.health)?;
        let redis = RedisConfig::try_from(&raw.redis)?;
        let fruit = FruitConfig::try_from(&raw.fruit)?;

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
