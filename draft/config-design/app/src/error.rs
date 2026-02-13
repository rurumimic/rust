use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ValidationError {
    #[error("logger.output=file requires a non-empty logger.file")]
    LoggerFileRequired,

    #[error("health.path must be non-empty")]
    HealthPathEmpty,

    #[error("health.path must start with '/'")]
    HealthPathNotAbsolute,

    #[error("health.timeout_ms must be greater than 0")]
    HealthTimeoutZero,

    #[error("redis.url must be non-empty")]
    RedisUrlEmpty,

    #[error("redis.pool_size must be greater than 0")]
    RedisPoolSizeZero,

    #[error("redis.connect_timeout_ms must be greater than 0")]
    RedisConnectTimeoutZero,
}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum AppConfigError {
    #[error("config load error: {0}")]
    Load(#[from] config::ConfigError),

    #[error("schema error: {0}")]
    Schema(#[from] schema::SchemaError),

    #[error("fruit error: {0}")]
    Fruit(#[from] fruits::FruitError),

    #[error("validation error: {0}")]
    Validation(#[from] ValidationError),
}

impl AppConfigError {
    #[must_use]
    pub fn user_message(&self) -> String {
        match self {
            AppConfigError::Load(_) => {
                "Failed to load config file. Check path and file format.".to_string()
            }
            AppConfigError::Schema(err) => format!("Config schema error: {err}"),
            AppConfigError::Fruit(err) => format!("Fruit config error: {err}"),
            AppConfigError::Validation(err) => format!("App config validation error: {err}"),
        }
    }
}
