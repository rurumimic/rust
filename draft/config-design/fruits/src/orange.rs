use config_schema::FruitSettingsRaw;
use serde::Deserialize;

use crate::FruitError;

/// Orange 전용 확장 옵션
#[derive(Debug, Clone, Deserialize)]
pub struct OrangeOptions {
    pub seedless: Option<bool>,
}

/// Orange 도메인 Config (완전 타입화)
#[derive(Debug, Clone)]
pub struct OrangeConfig {
    pub color: String,
    pub segments: i32,
    pub options: OrangeOptions,
}

impl OrangeConfig {
    pub const KNOWN_EXTRA_KEYS: &[&str] = &["segments", "options"];

    pub fn try_from_raw<F>(raw: &FruitSettingsRaw, warn_fn: F) -> Result<Self, FruitError>
    where
        F: Fn(&str),
    {
        let segments: i32 = raw.extract_required("segments")?;

        if segments <= 0 {
            return Err(FruitError::InvalidSegments(segments));
        }

        let options: OrangeOptions = raw
            .extract("options")?
            .unwrap_or(OrangeOptions { seedless: None });

        let unknown = raw.unknown_keys(Self::KNOWN_EXTRA_KEYS);
        raw.unknown_key_policy.handle_unknown(&unknown, "orange", warn_fn)?;

        Ok(OrangeConfig {
            color: raw.color.clone(),
            segments,
            options,
        })
    }
}
