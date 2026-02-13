//! Fruits domain crate.
//!
//! Contains fruit domain types with validation logic.
//! Uses raw schema types from core for deserialization.

mod apple;
mod banana;
mod config;
mod error;
mod orange;

pub use apple::AppleConfig;
pub use banana::BananaConfig;
pub use config::FruitConfig;
pub use error::FruitError;
pub use orange::OrangeConfig;

// Re-export schema types for convenience
pub use app::schema::{
    AppleOptions, AppleSettingsRaw, BananaOptions, BananaSettingsRaw, Curvature, FruitSettingsRaw,
    OrangeOptions, OrangeSettingsRaw, SchemaError,
};
