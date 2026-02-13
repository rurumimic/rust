use schema::{AppleOptions, AppleSettingsRaw};

use crate::FruitError;

/// Apple domain config.
#[derive(Debug, Clone)]
pub struct AppleConfig {
    pub color: String,
    pub sweetness: i32,
    pub options: AppleOptions,
}

impl TryFrom<&AppleSettingsRaw> for AppleConfig {
    type Error = FruitError;

    fn try_from(raw: &AppleSettingsRaw) -> Result<Self, Self::Error> {
        if !(0..=10).contains(&raw.sweetness) {
            return Err(FruitError::InvalidSweetness(raw.sweetness));
        }

        let unknown = raw.unknown_keys();
        raw.unknown_key_policy.handle_unknown(&unknown, "apple")?;

        Ok(AppleConfig {
            color: raw.color.clone(),
            sweetness: raw.sweetness,
            options: raw.options.clone(),
        })
    }
}
