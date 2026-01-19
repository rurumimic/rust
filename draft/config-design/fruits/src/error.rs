use config_schema::SchemaError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FruitError {
    #[error("schema error: {0}")]
    Schema(#[from] SchemaError),

    #[error("invalid sweetness level: {0} (must be 0-10)")]
    InvalidSweetness(i32),

    #[error("invalid curvature: {0}")]
    InvalidCurvature(String),

    #[error("invalid segment count: {0} (must be positive)")]
    InvalidSegments(i32),
}
