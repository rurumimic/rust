use config_schema::{AppleOptions, AppleSettingsRaw};

use crate::FruitError;

/// Apple 도메인 Config (완전 타입화)
#[derive(Debug, Clone)]
pub struct AppleConfig {
    pub color: String,
    pub sweetness: i32,
    pub options: AppleOptions,
}

impl AppleConfig {
    /// FruitSettingsRaw에서 AppleConfig로 변환
    pub fn try_from_raw(raw: &AppleSettingsRaw) -> Result<Self, FruitError> {
        // 1. 유효성 검증
        if !(0..=10).contains(&raw.sweetness) {
            return Err(FruitError::InvalidSweetness(raw.sweetness));
        }

        // 2. unknown 키 처리
        let unknown = raw.unknown_keys();
        raw.unknown_key_policy.handle_unknown(&unknown, "apple")?;

        Ok(AppleConfig {
            color: raw.color.clone(),
            sweetness: raw.sweetness,
            options: raw.options.clone(),
        })
    }
}
