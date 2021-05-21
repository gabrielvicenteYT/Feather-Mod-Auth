use actix_web::{web, HttpResponse, get};
use crate::state::HandlebarManager;
use crate::utils::error::WebsiteError;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct MainData {
    // This does not contain anything as of today.
}

#[get("/home")]
pub async fn display_home_page(state: web::Data<HandlebarManager>) -> Result<HttpResponse, WebsiteError> {
    Ok(HttpResponse::Ok().body(
        state
            .handlebars
            .read()
            .unwrap()
            .render_handlebar("main", &MainData {})?
    ))
}