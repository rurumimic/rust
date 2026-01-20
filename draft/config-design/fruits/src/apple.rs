use config_schema::{AppleOptionsRaw, AppleSettingsRaw};

use crate::FruitError;

/// Apple 도메인 옵션 (비즈니스 규칙 포함)
#[derive(Debug, Clone)]
pub struct AppleOptions {
    pub max_price: Option<u32>,
    pub season_only: Option<bool>,
}

impl AppleOptions {
    pub fn try_from_raw(raw: &AppleOptionsRaw) -> Result<Self, FruitError> {
        if let Some(max_price) = raw.max_price {
            if max_price == 0 {
                return Err(FruitError::InvalidMaxPrice(max_price));
            }
        }

        Ok(Self {
            max_price: raw.max_price,
            season_only: raw.season_only,
        })
    }
}

/// Apple 도메인 Config (완전 타입화)
#[derive(Debug, Clone)]
pub struct AppleConfig {
    pub color: String,
    pub sweetness: i32,
    pub options: AppleOptions,
}

impl AppleConfig {
    /// FruitSettingsRaw에서 AppleConfig로 변환
    pub fn try_from_raw<F>(raw: &AppleSettingsRaw, warn_fn: F) -> Result<Self, FruitError>
    where
        F: Fn(&str),
    {
        // 1. 유효성 검증
        if !(0..=10).contains(&raw.sweetness) {
            return Err(FruitError::InvalidSweetness(raw.sweetness));
        }

        // 2. unknown 키 처리
        let unknown = raw.unknown_keys();
        raw.unknown_key_policy.handle_unknown(&unknown, "apple", warn_fn)?;

        Ok(AppleConfig {
            color: raw.color.clone(),
            sweetness: raw.sweetness,
            options: AppleOptions::try_from_raw(&raw.options)?,
        })
    }
}
