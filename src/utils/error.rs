use actix_web::error;
use error::ResponseError;
use std::fmt;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WebsiteErrors {
    #[error("template rendering error")]
    RenderError(#[from] handlebars::RenderError),
}

impl ResponseError for WebsiteErrors {}
