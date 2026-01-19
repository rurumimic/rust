//! Fruits 도메인 크레이트
//!
//! 과일별 도메인 로직과 타입 안전한 Config 구조체를 정의합니다.

pub mod apple;
pub mod banana;
pub mod config;
pub mod error;
pub mod orange;

pub use apple::AppleConfig;
pub use banana::BananaConfig;
pub use config::FruitConfig;
pub use error::FruitError;
pub use orange::OrangeConfig;
