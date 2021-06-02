use crate::state::HandlebarManager;
use crate::utils::error::WebsiteError;
use actix_web::{get, web, HttpResponse};
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct MainData {
    // This does not contain anything as of today.
}

#[get("/home")]
pub async fn display_home_page(
    state: web::Data<HandlebarManager>,
) -> Result<HttpResponse, WebsiteError> {
    Ok(HttpResponse::Ok().body(
        state
            .handlebars
            .read()
            .unwrap()
            .render_handlebar("main", &MainData {})?,
    ))
}
