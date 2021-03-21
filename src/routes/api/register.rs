use actix_web::{post, web, HttpResponse};
use crate::models::user::{UserCreationRequest, User};
use sqlx::PgPool;
use crate::utils::error::WebsiteError;
use actix_web::http::StatusCode;

#[post("/register")]
pub async fn register(user: web::Json<UserCreationRequest>, pool: web::Data<PgPool>) -> Result<HttpResponse, WebsiteError> {
    User::create_user(user.0, pool.get_ref()).await?;
    Ok(HttpResponse::new(StatusCode::NO_CONTENT))
}