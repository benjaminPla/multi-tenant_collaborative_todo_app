mod application;
mod config;
mod domain;
mod infrastructure;
mod interface;
mod startup;

use config::Config;
use interface::router::app;
use startup::{database, http, server, state, telemetry};
use tokio::sync::watch;

#[tokio::main]
async fn main() {
    telemetry::init();
    let config = Config::from_env();

    let pool                       = database::connect(&config).await;
    let (shutdown_tx, shutdown_rx) = watch::channel(false);
    let state                      = state::build(pool, &config, shutdown_rx);

    server::serve(&config, http::service(app(state), &config), shutdown_tx).await;
}
