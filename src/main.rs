use std::sync::Arc;

use tracing::{error, info};

use rust_clean_project::{
    config::config_loader,
    infastructure::{axum_http::routers::http_serve::start, postgres::postgres_connection},
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let dotenvy_env = match config_loader::load() {
        Ok(env) => env,
        Err(e) => {
            error!("Failed to load ENV: {}", e);
            std::process::exit(1);
        }
    };

    info!("ENV has been loaded");

    let pg = match postgres_connection::establish_connection(&dotenvy_env.database.url) {
        Ok(pool) => pool,
        Err(e) => {
            error!("Failed to Establis Connection to Postgres: {}", e);
            std::process::exit(1)
        }
    };

    info!("Postgres Connection has been establish");

    start(Arc::new(dotenvy_env), Arc::new(pg))
        .await
        .expect("Failed to Start Server")
}
