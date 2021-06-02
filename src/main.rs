use actix_web::{App, HttpServer};
use anyhow::{Result};
use log::{error, info, LevelFilter};
use sqlx::PgPool;

use crate::state::{HandlebarManager};
use lazy_static::lazy_static;
use state::config::Settings;

use std::str::FromStr;

pub mod models;
pub mod routes;
pub mod state;
pub mod utils;

lazy_static! {
    static ref CONFIG: Settings = Settings::new().unwrap();
}

#[actix_web::main]
async fn main() -> Result<()> {

    env_logger::builder()
        .filter_level(LevelFilter::from_str(CONFIG.log.level.as_str())?)
        .init();

    //region state
    let db_pool = PgPool::connect(&CONFIG.database.url).await?;
    let handlebar_manager = HandlebarManager::default().register();
    //endregion
    if dotenv::dotenv().is_ok() {
        let bind_addr = format!(
            "{}:{}",
            &CONFIG.server.host,
            &CONFIG.server.port
        );

        let mut server = HttpServer::new(move || {
            // TODO: Find a way to automaticly discover new services instead of adding them to this file.
            App::new()
                .data(handlebar_manager.clone())
                .data(db_pool.clone())
                .configure(routes::routes_import)
        });

        server = server.bind(&bind_addr)?;

        info!("Starting server on {}...", bind_addr);
        server.run().await?;
        Ok(())
    } else {
        error!("There was an issue while trying to load server parameters.");
        Ok(())
    }
}
