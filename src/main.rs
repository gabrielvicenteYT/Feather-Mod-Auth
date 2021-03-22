use crate::state::State;
use crate::utils::handlebars_utils::HandlebarsUtils;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use anyhow::{Context, Result};
use log::{error, info, LevelFilter};
use sqlx::PgPool;


pub mod routes;
pub mod state;
pub mod utils;
pub mod models;
pub mod config;

#[actix_web::main]
async fn main() -> Result<()> {
    env_logger::builder().filter_level(LevelFilter::Info).init();
    let database_url =
        dotenv::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db_pool = PgPool::connect(&database_url).await?;
    let state = web::Data::new(State::default().register());
    if let Some(_) = dotenv::dotenv().ok() {
        let mut server = HttpServer::new(move || {
            let app = App::new();
            // TODO: Find a way to automaticly discover new services instead of adding them to this file.
            app
                .data(state.clone())
                .data(db_pool.clone())
                .service(routes::home::home_page)
                .service(routes::web::display_register_page)
                .service(routes::api::register::register)
        });
        let bind_addr = dotenv::var("BIND_ADDR")
            .context("The field `BIND_ADDR was not found in the ENV variables.`")?;


        server = server
            .bind(
                bind_addr.clone()
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
