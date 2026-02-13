use schema::{OrangeOptions, OrangeSettingsRaw};

use crate::FruitError;

/// Orange domain config.
#[derive(Debug, Clone)]
pub struct OrangeConfig {
    pub color: String,
    pub segments: i32,
    pub options: OrangeOptions,
}

impl TryFrom<OrangeSettingsRaw> for OrangeConfig {
    type Error = FruitError;

    fn try_from(raw: OrangeSettingsRaw) -> Result<Self, Self::Error> {
        if raw.segments <= 0 {
            return Err(FruitError::InvalidSegments(raw.segments));
        }

        let unknown = raw.unknown_keys();
        raw.unknown_key_policy.handle_unknown(&unknown, "orange")?;

        Ok(OrangeConfig {
            color: raw.color,
            segments: raw.segments,
            options: raw.options,
        })
    }
}
