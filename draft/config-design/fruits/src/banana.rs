use config_schema::{BananaOptions, BananaSettingsRaw, Curvature};

use crate::FruitError;

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
            options: raw.options.clone(),
        })
    }
}
