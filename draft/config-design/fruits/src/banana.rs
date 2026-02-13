use app::schema::{BananaOptions, BananaSettingsRaw, Curvature};

use crate::FruitError;

/// Banana domain config.
#[derive(Debug, Clone)]
pub struct BananaConfig {
    pub color: String,
    pub curvature: Curvature,
    pub options: BananaOptions,
}

impl BananaConfig {
    pub fn try_from_raw(raw: &BananaSettingsRaw) -> Result<Self, FruitError> {
        let unknown = raw.unknown_keys();
        raw.unknown_key_policy.handle_unknown(&unknown, "banana")?;

        Ok(BananaConfig {
            color: raw.color.clone(),
            curvature: raw.curvature.clone(),
            options: raw.options.clone(),
        })
    }
}
