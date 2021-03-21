use crate::state::State;
use crate::utils::error::WebsiteErrors;
use actix_web::{get, web, HttpResponse, Result};
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};
use std::borrow::Borrow;

#[derive(Serialize, Deserialize)]
pub struct RegisterData {
    // This does not contain anything as of today.
}

#[get("/test/register")]
pub async fn display_register_page(state: web::Data<State>) -> Result<HttpResponse, WebsiteErrors> {
    Ok(HttpResponse::Ok().body(
        state
            .handlebars
            .read()
            .unwrap()
            .render_handlebar("register", &RegisterData {})?
    ))
}
