use config_schema::FruitSettingsRaw;
use serde::Deserialize;

use crate::FruitError;

/// Banana 곡률
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Curvature {
    Low,
    Medium,
    High,
}

impl std::str::FromStr for Curvature {
    type Err = FruitError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "low" => Ok(Curvature::Low),
            "medium" => Ok(Curvature::Medium),
            "high" => Ok(Curvature::High),
            other => Err(FruitError::InvalidCurvature(other.to_string())),
        }
    }
}

/// Banana 전용 확장 옵션
#[derive(Debug, Clone, Deserialize)]
pub struct BananaOptions {
    pub ripeness: Option<f64>,
}

/// Banana 도메인 Config (완전 타입화)
#[derive(Debug, Clone)]
pub struct BananaConfig {
    pub color: String,
    pub curvature: Curvature,
    pub options: BananaOptions,
}

impl BananaConfig {
    pub const KNOWN_EXTRA_KEYS: &[&str] = &["curvature", "options"];

    pub fn try_from_raw<F>(raw: &FruitSettingsRaw, warn_fn: F) -> Result<Self, FruitError>
    where
        F: Fn(&str),
    {
        let curvature_str: String = raw.extract_required("curvature")?;
        let curvature = curvature_str.parse()?;

        let options: BananaOptions = raw
            .extract("options")?
            .unwrap_or(BananaOptions { ripeness: None });

        let unknown = raw.unknown_keys(Self::KNOWN_EXTRA_KEYS);
        raw.unknown_key_policy.handle_unknown(&unknown, "banana", warn_fn)?;

        Ok(BananaConfig {
            color: raw.color.clone(),
            curvature,
            options,
        })
    }
}
