use actix_web::{App, get, HttpResponse, HttpServer, post, Responder, web};
use anyhow::{Context, Result};
use log::{error, info, LevelFilter};
use sqlx::PgPool;

use state::config::Settings;
use state::handlebars_utils::HandlebarsUtils;
use crate::state::{HandlebarManager, ConfigManager};
use std::str::FromStr;

pub mod routes;
pub mod utils;
pub mod models;
pub mod state;

#[actix_web::main]
async fn main() -> Result<()> {
    let config = ConfigManager::from(Settings::new()?);

    env_logger::builder().filter_level(LevelFilter::from_str(config.config.log.level.as_str())?).init();

    //region state
    let db_pool = PgPool::connect(&config.config.database.url).await?;
    let handlebar_manager = HandlebarManager::default(config.config.clone()).register();
    //endregion
    if let Some(_) = dotenv::dotenv().ok() {

        let bind_addr = format!("{}:{}", config.config.server.host.clone(), config.config.server.port.clone());

        let mut server = HttpServer::new(move || {
            // TODO: Find a way to automaticly discover new services instead of adding them to this file.
            App::new()
                .data(handlebar_manager.clone())
                .data(db_pool.clone())
                .data(config.clone())
                .configure(routes::routes_import)
        });

        server = server
            .bind(
                &bind_addr
            )?;

        info!("Starting server on {}...", bind_addr);
        server
            .run()
            .await?;
        Ok(())
    } else {
        error!("There was an issue while trying to load server parameters.");
        Ok(())
    }
}
