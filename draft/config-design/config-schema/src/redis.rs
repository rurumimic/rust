use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct RedisSettingsRaw {
    #[serde(default = "default_redis_url")]
    pub url: String,
    #[serde(default = "default_redis_pool_size")]
    pub pool_size: u32,
    #[serde(default = "default_redis_connect_timeout_ms")]
    pub connect_timeout_ms: u64,
}

fn default_redis_url() -> String {
    "redis://127.0.0.1:6379".to_string()
}

fn default_redis_pool_size() -> u32 {
    10
}

fn default_redis_connect_timeout_ms() -> u64 {
    2000
}

impl Default for RedisSettingsRaw {
    fn default() -> Self {
        RedisSettingsRaw {
            url: default_redis_url(),
            pool_size: default_redis_pool_size(),
            connect_timeout_ms: default_redis_connect_timeout_ms(),
        }
    }
}
