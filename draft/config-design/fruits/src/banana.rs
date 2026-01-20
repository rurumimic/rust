use config_schema::{BananaOptionsRaw, BananaSettingsRaw, Curvature};

use crate::FruitError;

/// Banana 도메인 옵션 (비즈니스 규칙 포함)
#[derive(Debug, Clone)]
pub struct BananaOptions {
    pub ripeness: Option<f64>,
}

impl BananaOptions {
    pub fn try_from_raw(raw: &BananaOptionsRaw) -> Result<Self, FruitError> {
        if let Some(ripeness) = raw.ripeness {
            if !(0.0..=1.0).contains(&ripeness) {
                return Err(FruitError::InvalidRipeness(ripeness));
            }
        }

        Ok(Self {
            ripeness: raw.ripeness,
        })
    }
}

/// Banana 도메인 Config (완전 타입화)
#[derive(Debug, Clone)]
pub struct BananaConfig {
    pub color: String,
    pub curvature: Curvature,
    pub options: BananaOptions,
}

impl BananaConfig {
    pub fn try_from_raw<F>(raw: &BananaSettingsRaw, warn_fn: F) -> Result<Self, FruitError>
    where
        F: Fn(&str),
    {
        let unknown = raw.unknown_keys();
        raw.unknown_key_policy.handle_unknown(&unknown, "banana", warn_fn)?;

        Ok(BananaConfig {
            color: raw.color.clone(),
            curvature: raw.curvature.clone(),
            options: BananaOptions::try_from_raw(&raw.options)?,
        })
    }
}
