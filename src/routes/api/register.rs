use crate::models::user::{User, UserCreationRequest};
use crate::utils::error::WebsiteError::CustomError;
use crate::utils::error::{Error, WebsiteError};
use actix_web::http::StatusCode;
use actix_web::{post, web, HttpResponse};
use sqlx::PgPool;

#[post("/register")]
pub async fn register(
    user: web::Json<UserCreationRequest>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, WebsiteError> {
    // Check for duplicate username
    if !User::check_username_availability(&user.0.username, pool.get_ref()).await? {
        return Err(CustomError(
            Error {
                ok: false,
                name: "username_taken",
                description: "The username you are trying to register does not work.",
                issues: None,
            },
            StatusCode::CONFLICT,
        ));
    }
    User::create_user(user.0, pool.get_ref()).await?;
    Ok(HttpResponse::new(StatusCode::NO_CONTENT))
}
