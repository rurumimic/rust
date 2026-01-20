use config_schema::SchemaError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FruitError {
    #[error("schema error: {0}")]
    Schema(#[from] SchemaError),

    #[error("invalid sweetness level: {0} (must be 0-10)")]
    InvalidSweetness(i32),

    #[error("invalid max price: {0} (must be positive)")]
    InvalidMaxPrice(u32),

    #[error("invalid ripeness: {0} (must be between 0.0 and 1.0)")]
    InvalidRipeness(f64),

    #[error("invalid segment count: {0} (must be positive)")]
    InvalidSegments(i32),
}
