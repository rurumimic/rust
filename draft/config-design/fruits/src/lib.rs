//! Fruits domain crate.
//!
//! Defines domain logic and type-safe config structures for fruits.

pub mod apple;
pub mod banana;
pub mod config;
pub mod error;
pub mod orange;

pub use apple::AppleConfig;
pub use banana::BananaConfig;
pub use config::FruitConfig;
pub use error::FruitError;
pub use config_schema::SchemaError;
pub use orange::OrangeConfig;
