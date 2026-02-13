use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum SchemaError {
    #[error("unknown fruit kind: {0}")]
    UnknownFruitKind(String),

    #[error("missing required field: {0}")]
    MissingField(String),

    #[error("invalid field type for '{field}': expected {expected}, got {actual}")]
    InvalidFieldType {
        field: String,
        expected: String,
        actual: String,
    },

    #[error("unknown keys not allowed: {0:?}")]
    UnknownKeys(Vec<String>),

    #[error("validation failed: {0}")]
    ValidationFailed(String),
}
