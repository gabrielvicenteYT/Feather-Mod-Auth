use actix_web::error;
use error::ResponseError;
use std::fmt;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WebsiteError {
    #[error("template rendering error")]
    RenderError(#[from] handlebars::RenderError),
    #[error("An error occurred while hasing the password.")]
    CryptoError(),
    #[error("An error occurred while doing an operation to the database.")]
    DatabaseError(#[from] sqlx::Error)
}

impl ResponseError for WebsiteError {}
