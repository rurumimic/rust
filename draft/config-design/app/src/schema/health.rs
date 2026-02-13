use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct HealthSettingsRaw {
    pub enabled: bool,
    pub path: String,
    pub timeout_ms: u64,
}

impl Default for HealthSettingsRaw {
    fn default() -> Self {
        HealthSettingsRaw {
            enabled: true,
            path: String::from("/health"),
            timeout_ms: 2000,
        }
    }
}
