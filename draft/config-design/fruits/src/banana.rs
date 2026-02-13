use schema::{BananaOptions, BananaSettingsRaw, Curvature};

use crate::FruitError;

/// Banana domain config.
#[derive(Debug, Clone)]
pub struct BananaConfig {
    pub color: String,
    pub curvature: Curvature,
    pub options: BananaOptions,
}

impl TryFrom<&BananaSettingsRaw> for BananaConfig {
    type Error = FruitError;

    fn try_from(raw: &BananaSettingsRaw) -> Result<Self, Self::Error> {
        let unknown = raw.unknown_keys();
        raw.unknown_key_policy.handle_unknown(&unknown, "banana")?;

        Ok(BananaConfig {
            color: raw.color.clone(),
            curvature: raw.curvature.clone(),
            options: raw.options.clone(),
        })
    }
}
