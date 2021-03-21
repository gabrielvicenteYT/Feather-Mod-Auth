use crate::state::State;
use crate::utils::handlebars_utils::HandlebarsUtils;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use anyhow::{Context, Result};
use log::{error, info, LevelFilter};

pub mod routes;
pub mod state;
pub mod utils;

#[actix_web::main]
async fn main() -> Result<()> {
    env_logger::builder().filter_level(LevelFilter::Info).init();
    let state = web::Data::new(State::default().register());
    if let Some(_) = dotenv::dotenv().ok() {
        Ok(HttpServer::new(move || {
            let app = App::new();
            app.app_data(state.clone())
                .service(routes::home::home_page)
                .service(routes::web::display_register_page)
        })
        .bind(
            dotenv::var("BIND_ADDR")
                .context("The field `BIND_ADDR was not found in the ENV variables.`")?,
        )?
        .run()
        .await?)
    } else {
        error!("There was an issue while trying to load server parameters.");
        Ok(())
    }
}
