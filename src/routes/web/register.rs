use crate::state::HandlebarManager;
use crate::utils::error::WebsiteError;
use actix_web::{get, web, HttpResponse, Result};
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};
use std::borrow::Borrow;

#[derive(Serialize, Deserialize)]
pub struct RegisterData {
    // This does not contain anything as of today.
}

#[get("/register")]
pub async fn display_register_page(
    state: web::Data<HandlebarManager>,
) -> Result<HttpResponse, WebsiteError> {
    Ok(HttpResponse::Ok().body(
        state
            .handlebars
            .read()
            .unwrap()
            .render_handlebar("main", &RegisterData {})?,
    ))
}
