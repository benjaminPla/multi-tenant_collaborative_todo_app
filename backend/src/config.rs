use serde::Deserialize;
use std::net::IpAddr;

#[derive(Deserialize)]
pub struct Config {
    pub database_url:            String,
    pub db_acquire_timeout_secs: u64,
    pub db_idle_timeout_secs:    u64,
    pub db_max_connections:      u32,
    pub db_min_connections:      u32,
    pub host:                    IpAddr,
    pub http_body_limit_bytes:   usize,
    pub http_body_timeout_secs:  u64,
    pub http_concurrency_limit:  usize,
    pub http_timeout_secs:       u64,
    pub port:                    u16,
    pub session_ttl_secs:        u32,
    pub sse_max_connections:     usize,
}
