use config_schema::FruitSettingsRaw;
use serde::Deserialize;

use crate::FruitError;

/// Apple 전용 확장 옵션
#[derive(Debug, Clone, Deserialize)]
pub struct AppleOptions {
    pub max_price: Option<u32>,
    pub season_only: Option<bool>,
}

/// Apple 도메인 Config (완전 타입화)
#[derive(Debug, Clone)]
pub struct AppleConfig {
    pub color: String,
    pub sweetness: i32,
    pub options: AppleOptions,
}

impl AppleConfig {
    /// 알려진 extra 키 목록
    pub const KNOWN_EXTRA_KEYS: &[&str] = &["sweetness", "options"];

    /// FruitSettingsRaw에서 AppleConfig로 변환
    pub fn try_from_raw<F>(raw: &FruitSettingsRaw, warn_fn: F) -> Result<Self, FruitError>
    where
        F: Fn(&str),
    {
        // 1. 필수/옵션 필드 추출
        let sweetness: i32 = raw.extract_required("sweetness")?;

        // 2. 유효성 검증
        if !(0..=10).contains(&sweetness) {
            return Err(FruitError::InvalidSweetness(sweetness));
        }

        // 3. 확장 옵션 추출 (options 객체)
        let options: AppleOptions = raw
            .extract("options")?
            .unwrap_or(AppleOptions {
                max_price: None,
                season_only: None,
            });

        // 4. unknown 키 처리
        let unknown = raw.unknown_keys(Self::KNOWN_EXTRA_KEYS);
        raw.unknown_key_policy.handle_unknown(&unknown, "apple", warn_fn)?;

        Ok(AppleConfig {
            color: raw.color.clone(),
            sweetness,
            options,
        })
    }
}
