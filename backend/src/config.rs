use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub database_url:            String,
    pub db_acquire_timeout_secs: u64,
    pub db_idle_timeout_secs:    u64,
    pub db_max_connections:      u32,
    pub db_min_connections:      u32,
    pub port:                    u16,
    pub session_ttl_secs:        u32,
}
