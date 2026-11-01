use tracing_subscriber::EnvFilter;

pub fn init() {
    let logs = tracing_subscriber::fmt().with_env_filter(EnvFilter::from_default_env());
    match std::env::var("LOG_FORMAT").as_deref() {
        Ok("json") => logs.json().init(),
        _          => logs.init(),
    }
}
