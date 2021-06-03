
use crate::utils::error::WebsiteError;
use actix_web::{get, web, HttpResponse, Result};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RegisterData {
    // This does not contain anything as of today.
}

#[get("/register")]
pub async fn display_register_page(
) -> Result<HttpResponse, WebsiteError> {
    Ok(HttpResponse::Ok().body(
        ""
    ))
}
