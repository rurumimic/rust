use schema::FruitSettingsRaw;

use crate::{AppleConfig, BananaConfig, FruitError, OrangeConfig};

/// Fruit config enum (type-safe variants).
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum FruitConfig {
    Apple(AppleConfig),
    Banana(BananaConfig),
    Orange(OrangeConfig),
}

impl TryFrom<FruitSettingsRaw> for FruitConfig {
    type Error = FruitError;

    fn try_from(raw: FruitSettingsRaw) -> Result<Self, Self::Error> {
        match raw {
            FruitSettingsRaw::Apple(raw) => Ok(FruitConfig::Apple(raw.try_into()?)),
            FruitSettingsRaw::Banana(raw) => Ok(FruitConfig::Banana(raw.try_into()?)),
            FruitSettingsRaw::Orange(raw) => Ok(FruitConfig::Orange(raw.try_into()?)),
        }
    }
}

impl FruitConfig {
    /// Returns fruit kind name.
    #[must_use]
    pub fn kind(&self) -> &'static str {
        match self {
            FruitConfig::Apple(_) => "apple",
            FruitConfig::Banana(_) => "banana",
            FruitConfig::Orange(_) => "orange",
        }
    }

    /// Returns color (shared field).
    #[must_use]
    pub fn color(&self) -> &str {
        match self {
            FruitConfig::Apple(c) => &c.color,
            FruitConfig::Banana(c) => &c.color,
            FruitConfig::Orange(c) => &c.color,
        }
    }
}
