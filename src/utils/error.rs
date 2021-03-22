use actix_web::{error, HttpResponse};
use std::fmt;
use thiserror::Error;
use actix_web::dev::HttpResponseBuilder;
use actix_web::http::StatusCode;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Error {
    pub(crate) ok: bool,
    pub(crate) name: &'static str,
    pub(crate) description: &'static str
}

#[derive(Error, Debug)]
pub enum WebsiteError {
    #[error("template rendering error")]
    RenderError(#[from] handlebars::RenderError),
    #[error("An error occurred while hasing the password.")]
    CryptoError(),
    #[error("An error occurred while doing an operation to the database.")]
    DatabaseError(#[from] sqlx::Error),
    #[error("An error occurred while processing your request.")]
    CustomError(Error, StatusCode)
}
impl WebsiteError {
    fn build_error(&self) -> Error {
        match self {
            WebsiteError::RenderError(_) | WebsiteError::CryptoError() | WebsiteError::DatabaseError(_) => Error {
                ok: false,
                name: "internal_error",
                description: "There was an issue while executing this action."
            },

            WebsiteError::CustomError(error, _) => error.clone()
        }
    }
}
impl error::ResponseError for WebsiteError {
    fn status_code(&self) -> StatusCode {
        match self {
            WebsiteError::RenderError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            WebsiteError::CryptoError() => StatusCode::INTERNAL_SERVER_ERROR,
            WebsiteError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            WebsiteError::CustomError(_, status) => status.clone(),
        }
    }
    fn error_response(&self) -> HttpResponse {
        return HttpResponseBuilder::new(self.status_code())
            .json(self.build_error());
    }


}