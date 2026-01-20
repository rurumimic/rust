use config_schema::FruitSettingsRaw;

use crate::{AppleConfig, BananaConfig, FruitError, OrangeConfig};

/// 과일 종류별 Config enum (타입 안전한 분기)
#[derive(Debug, Clone)]
pub enum FruitConfig {
    Apple(AppleConfig),
    Banana(BananaConfig),
    Orange(OrangeConfig),
}

impl FruitConfig {
    /// FruitSettingsRaw에서 FruitConfig로 변환
    ///
    /// kind 필드에 따라 적절한 Config 타입으로 변환합니다.
    pub fn try_from_raw(raw: &FruitSettingsRaw) -> Result<Self, FruitError> {
        match raw {
            FruitSettingsRaw::Apple(raw) => {
                Ok(FruitConfig::Apple(AppleConfig::try_from_raw(raw)?))
            }
            FruitSettingsRaw::Banana(raw) => {
                Ok(FruitConfig::Banana(BananaConfig::try_from_raw(raw)?))
            }
            FruitSettingsRaw::Orange(raw) => {
                Ok(FruitConfig::Orange(OrangeConfig::try_from_raw(raw)?))
            }
        }
    }

    /// 과일 종류 이름 반환
    pub fn kind(&self) -> &'static str {
        match self {
            FruitConfig::Apple(_) => "apple",
            FruitConfig::Banana(_) => "banana",
            FruitConfig::Orange(_) => "orange",
        }
    }

    /// 색상 반환 (공통 필드)
    pub fn color(&self) -> &str {
        match self {
            FruitConfig::Apple(c) => &c.color,
            FruitConfig::Banana(c) => &c.color,
            FruitConfig::Orange(c) => &c.color,
        }
    }
}
