use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct RedisSettingsRaw {
    pub url: String,
    pub pool_size: u32,
    pub connect_timeout_ms: u64,
}

impl Default for RedisSettingsRaw {
    fn default() -> Self {
        RedisSettingsRaw {
            url: String::from("redis://127.0.0.1:6379"),
            pool_size: 10,
            connect_timeout_ms: 2000,
        }
    }
}
