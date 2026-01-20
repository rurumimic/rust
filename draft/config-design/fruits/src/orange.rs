use config_schema::{OrangeOptions, OrangeSettingsRaw};

use crate::FruitError;

/// Orange domain config.
#[derive(Debug, Clone)]
pub struct OrangeConfig {
    pub color: String,
    pub segments: i32,
    pub options: OrangeOptions,
}

impl OrangeConfig {
    pub fn try_from_raw(raw: &OrangeSettingsRaw) -> Result<Self, FruitError> {
        if raw.segments <= 0 {
            return Err(FruitError::InvalidSegments(raw.segments));
        }

        let unknown = raw.unknown_keys();
        raw.unknown_key_policy.handle_unknown(&unknown, "orange")?;

        Ok(OrangeConfig {
            color: raw.color.clone(),
            segments: raw.segments,
            options: raw.options.clone(),
        })
    }
}
