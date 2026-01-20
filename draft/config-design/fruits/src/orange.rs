use config_schema::{OrangeOptionsRaw, OrangeSettingsRaw};

use crate::FruitError;

/// Orange 도메인 옵션 (비즈니스 규칙 포함)
#[derive(Debug, Clone)]
pub struct OrangeOptions {
    pub seedless: Option<bool>,
}

impl OrangeOptions {
    pub fn try_from_raw(raw: &OrangeOptionsRaw) -> Result<Self, FruitError> {
        Ok(Self {
            seedless: raw.seedless,
        })
    }
}

/// Orange 도메인 Config (완전 타입화)
#[derive(Debug, Clone)]
pub struct OrangeConfig {
    pub color: String,
    pub segments: i32,
    pub options: OrangeOptions,
}

impl OrangeConfig {
    pub fn try_from_raw<F>(raw: &OrangeSettingsRaw, warn_fn: F) -> Result<Self, FruitError>
    where
        F: Fn(&str),
    {
        if raw.segments <= 0 {
            return Err(FruitError::InvalidSegments(raw.segments));
        }

        let unknown = raw.unknown_keys();
        raw.unknown_key_policy.handle_unknown(&unknown, "orange", warn_fn)?;

        Ok(OrangeConfig {
            color: raw.color.clone(),
            segments: raw.segments,
            options: OrangeOptions::try_from_raw(&raw.options)?,
        })
    }
}
