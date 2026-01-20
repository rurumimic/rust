use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

use crate::UnknownKeyPolicy;

/// Raw schema for fruit settings (input boundary).
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum FruitSettingsRaw {
    Apple(AppleSettingsRaw),
    Banana(BananaSettingsRaw),
    Orange(OrangeSettingsRaw),
}

/// Banana curvature.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Curvature {
    Low,
    Medium,
    High,
}

/// Apple-specific options.
#[derive(Debug, Clone, Deserialize, Default)]
pub struct AppleOptions {
    pub max_price: Option<u32>,
    pub season_only: Option<bool>,
}

/// Banana-specific options.
#[derive(Debug, Clone, Deserialize, Default)]
pub struct BananaOptions {
    pub ripeness: Option<f64>,
}

/// Orange-specific options.
#[derive(Debug, Clone, Deserialize, Default)]
pub struct OrangeOptions {
    pub seedless: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AppleSettingsRaw {
    pub color: String,
    pub sweetness: i32,
    #[serde(default)]
    pub options: AppleOptions,
    #[serde(default)]
    pub unknown_key_policy: UnknownKeyPolicy,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

impl AppleSettingsRaw {
    pub fn unknown_keys(&self) -> Vec<String> {
        self.extra.keys().cloned().collect()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct BananaSettingsRaw {
    pub color: String,
    pub curvature: Curvature,
    #[serde(default)]
    pub options: BananaOptions,
    #[serde(default)]
    pub unknown_key_policy: UnknownKeyPolicy,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

impl BananaSettingsRaw {
    pub fn unknown_keys(&self) -> Vec<String> {
        self.extra.keys().cloned().collect()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct OrangeSettingsRaw {
    pub color: String,
    pub segments: i32,
    #[serde(default)]
    pub options: OrangeOptions,
    #[serde(default)]
    pub unknown_key_policy: UnknownKeyPolicy,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

impl OrangeSettingsRaw {
    pub fn unknown_keys(&self) -> Vec<String> {
        self.extra.keys().cloned().collect()
    }
}
