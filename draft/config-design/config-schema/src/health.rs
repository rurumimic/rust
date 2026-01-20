use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct HealthSettingsRaw {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default = "default_health_path")]
    pub path: String,
    #[serde(default = "default_health_timeout_ms")]
    pub timeout_ms: u64,
}

fn default_health_path() -> String {
    "/health".to_string()
}

fn default_health_timeout_ms() -> u64 {
    2000
}

impl Default for HealthSettingsRaw {
    fn default() -> Self {
        HealthSettingsRaw {
            enabled: true,
            path: default_health_path(),
            timeout_ms: default_health_timeout_ms(),
        }
    }
}
