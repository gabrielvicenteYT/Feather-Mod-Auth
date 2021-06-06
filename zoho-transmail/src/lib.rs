use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use thiserror::Error;
const BASE_URL: &'static str = "https://api.transmail.com/v1.1/email";

#[derive(Error)]
pub enum Error {
    #[error("An error occurred while making the request")]
    NetworkError(#[from] reqwest::Error),
    #[error("The request to the transmail service failed with error code {}")]
    RequestError(u16),
}
/// Credentials that are gonna be used to authenticate to the transmail service.
pub struct Credentials {
    pub bounce_return_path: String,
    pub send_token: String,
}

/// This an async compatible wrapper for the transmail service.
///
pub struct MailSender {
    credentials: Credentials,
    client: Client,
}

impl MailSender {
    pub fn new(credentials: Credentials) -> Self {
        Self {
            credentials,
            client: Client::new(),
        }
    }
    pub async fn send_email(&self, req: ReqBody) -> Result<(), Error> {
        // Generate the body

        let res = self
            .client
            .post(BASE_URL)
            .header(
                "Authorization",
                format!("Zoho-enczapikey {}", self.credentials.send_token),
            )
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&req))
            .send()
            .await?;
        if res.status() != StatusCode::OK {
            // An error occurred
            return Err(Error::RequestError(res.status().as_u16()));
        }
        // Everything worked fine, return ok
        Ok(())
    }
}
#[derive(Serialize, Deserialize)]
struct ReqBody {
    bounce_address: String,
    from: EmailAddress,
    to: Vec<ToBody>,
    subject: String,
    textbody: Option<String>,
    htmlbody: Option<String>,
    inline_images: Vec<Image>,
    attachments: Vec<Attachments>,
}
#[derive(Serialize, Deserialize)]
struct EmailAddress {
    address: String,
    name: String,
}
#[derive(Serialize, Deserialize)]
struct ToBody {
    email_address: EmailAddress,
}
#[derive(Serialize, Deserialize)]
struct Image {
    mime_type: String,
    content: String,
    cid: String,
}
#[derive(Serialize, Deserialize)]
struct Attachments {
    mime_type: String,
    content: String,
    name: String,
}
