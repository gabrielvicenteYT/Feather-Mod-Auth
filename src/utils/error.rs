use actix_web::dev::HttpResponseBuilder;
use actix_web::http::StatusCode;
use actix_web::{error, HttpResponse};
use serde::Serialize;

use thiserror::Error;

#[derive(Serialize, Debug, Clone)]
pub struct Error {
    pub ok: bool,
    pub name: &'static str,
    pub description: &'static str,
    pub issues: Option<Vec<ValidationIssue>>,
}
#[derive(Serialize, Clone, Debug)]
pub struct ValidationIssue {
    pub field: String,
    pub error_name: String,
    pub error_description: String,
}

#[derive(Error, Debug)]
pub enum WebsiteError {
    #[error("template rendering error")]
    RenderError(#[from] askama::Error),
    #[error("An error occurred while hasing the password.")]
    CryptoError(),
    #[error("An error occurred while doing an operation to the database.")]
    DatabaseError(#[from] sqlx::Error),
    #[error("An error occurred while processing your request.")]
    CustomError(Error, StatusCode),
    #[error("There are errors with the function input.")]
    ValidationError(Vec<ValidationIssue>),
}
impl WebsiteError {
    fn build_error(&self) -> Error {
        match self {
            WebsiteError::RenderError(_)
            | WebsiteError::CryptoError()
            | WebsiteError::DatabaseError(_) => Error {
                ok: false,
                name: "internal_error",
                description: "There was an issue while executing this action.",
                issues: None,
            },

            WebsiteError::CustomError(error, _) => error.clone(),
            WebsiteError::ValidationError(issues) => Error {
                ok: false,
                name: "invalid_input",
                description:
                    "There was issues with your inputs. Please check them in the `issues` field.",
                issues: Some(issues.clone()),
            },
        }
    }
}
impl error::ResponseError for WebsiteError {
    fn status_code(&self) -> StatusCode {
        match self {
            WebsiteError::RenderError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            WebsiteError::CryptoError() => StatusCode::INTERNAL_SERVER_ERROR,
            WebsiteError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            WebsiteError::CustomError(_, status) => *status,
            WebsiteError::ValidationError(_) => StatusCode::BAD_REQUEST,
        }
    }
    fn error_response(&self) -> HttpResponse {
        return HttpResponseBuilder::new(self.status_code()).json(self.build_error());
    }
}
