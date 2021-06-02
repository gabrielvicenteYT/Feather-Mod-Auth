use crate::utils::error::WebsiteError;
use async_trait::async_trait;
use sqlx::PgPool;

pub mod oauth;

#[async_trait]
pub trait Validation {
    type FinalValue;
    /// Validate the input of the file, possibly modify input for normalization.
    /// This function has to return a [WebsiteError::ValidationError] with the vector containing all discovered issues.
    fn validate_input(&mut self) -> Result<(), WebsiteError>;
}
