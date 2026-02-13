use app::schema::FruitSettingsRaw;

use crate::{AppleConfig, BananaConfig, FruitError, OrangeConfig};

/// Fruit config enum (type-safe variants).
#[derive(Debug, Clone)]
pub enum FruitConfig {
    Apple(AppleConfig),
    Banana(BananaConfig),
    Orange(OrangeConfig),
}

impl FruitConfig {
    /// Convert from `FruitSettingsRaw` based on the `kind` tag.
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

    /// Returns fruit kind name.
    pub fn kind(&self) -> &'static str {
        match self {
            FruitConfig::Apple(_) => "apple",
            FruitConfig::Banana(_) => "banana",
            FruitConfig::Orange(_) => "orange",
        }
    }

    /// Returns color (shared field).
    pub fn color(&self) -> &str {
        match self {
            FruitConfig::Apple(c) => &c.color,
            FruitConfig::Banana(c) => &c.color,
            FruitConfig::Orange(c) => &c.color,
        }
    }
}
