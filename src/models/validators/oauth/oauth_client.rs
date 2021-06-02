use crate::models::oauth::oauth_client::OAuthClient;
use crate::models::validators::Validation;
use crate::utils::error::{ValidationIssue, WebsiteError};
use crate::utils::id::{generate_id, generate_secret};
use crate::utils::password_hashing::hash_password;
use async_trait::async_trait;

use lazy_static::lazy_static;
use regex::Regex;
use serde::Deserialize;
use sqlx::PgPool;

use url::Url;
use sqlx::FromRow;
use uuid::Uuid;
use sqlx::postgres::PgRow;

lazy_static! {
    static ref HOST_REGEX: Regex = Regex::new(r"^([a-z0-9]+(-[a-z0-9]+)*\.)+[a-z]{2,}$").unwrap();
}

#[derive(Deserialize)]
pub struct OAuthClientCreationRequest {
    name: String,
    /// Having an empty description is possible, but not recommended.
    description: String,
    privacy_policy: Option<String>,
    terms_of_service: Option<String>,
    js_domains: Option<Vec<String>>,
    callback_urls: Option<Vec<String>>,
    /// The icon has a specific requirement for the link.
    /// It has to be hosted on an authorized domain as set in the configuration file
    icon: Option<String>,
}

#[async_trait]
impl Validation for OAuthClientCreationRequest {
    type FinalValue = OAuthClient;

    fn validate_input(&mut self) -> Result<(), WebsiteError> {
        let mut errors: Vec<ValidationIssue> = vec![];
        // Check the icon
        if let Some(icon) = &self.icon {
            if let Ok(url) = Url::parse(icon) {
                if url.host_str().is_some()
                    && !crate::CONFIG
                        .general
                        .icon_domain
                        .contains(&url.host_str().unwrap().to_string())
                {
                    // The url is coming from an invalid domain
                    errors.push(ValidationIssue {
                        field: "icon".to_string(),
                        error_name: "invalid_domain".to_string(),
                        error_description:
                        "This domain is not present in the authorized domains for this organization"
                            .to_string(),
                    });
                }
            } else {
                errors.push(ValidationIssue {
                    field: "icon".to_string(),
                    error_name: "invalid_url".to_string(),
                    error_description: "The url isn't valid.".to_string(),
                });
            }
        }
        // Check if the allback urls are valid ones
        if let Some(urls) = &self.callback_urls {
            for url in urls {
                match Url::parse(url) {
                    Ok(_) => {}
                    Err(_) => {
                        errors.push(ValidationIssue {
                            field: "callback_urls".to_string(),
                            error_name: "invalid_url".to_string(),
                            error_description: format!("The url {} isn't valid.", url),
                        });
                    }
                };
            }
        }
        // Check if the domains are a valid hostname.
        if let Some(hosts) = &self.js_domains {
            for host in hosts {
                if !HOST_REGEX.is_match(&*host) {
                    errors.push(ValidationIssue {
                        field: "js_domains".to_string(),
                        error_name: "invalid_domain".to_string(),
                        error_description: format!("The domain {} is not a valid domain", host),
                    })
                }
            }
        }
        if !errors.is_empty() {
            Err(WebsiteError::ValidationError(errors))
        } else {
            Ok(())
        }
    }
}
impl OAuthClientCreationRequest {
    pub async fn insert(
        &self,
        user_id: Uuid,
        pool: &PgPool,
    ) -> Result<OAuthClient, WebsiteError> {
        // Generate elements needed:
        let id = generate_id(12);
        let secret = generate_secret(24);
        let secret_hash = hash_password(secret.clone())?;
        let js_domains: Option<&[String]> = match &self.js_domains {
            Some(e) => Some(e.as_slice()),
            None => None
        };
        let callback_urls: Option<&[String]> = match &self.callback_urls {
            Some(e) => Some(e.as_slice()),
            None => None
        };
        let obj: PgRow = sqlx::query_as!(
            OAuthClient,
            r#"
            INSERT INTO minos.oauth_clients
                (id, secret, owner, name, description, privacy_policy, terms_of_service, icon, js_domains, callback_urls)
            VALUES
                ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        "#,
        id, secret_hash.into(), user_id, self.name, self.description, self.privacy_policy, self.terms_of_service, self.icon,js_domains, callback_urls)
            .fetch_one(pool).await?;
        let mut obj = OAuthClient::from_row(&obj)?;
        obj.secret = Some(secret);
        Ok(obj)
    }
}
