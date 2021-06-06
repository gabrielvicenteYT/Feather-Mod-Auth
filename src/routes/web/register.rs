use crate::utils::error::WebsiteError;
use actix_web::{get, HttpResponse, Result};
use askama::Template;
use derive_more::Constructor;

#[derive(Template, Constructor)]
#[template(path = "register_prompt.html")]
pub struct RegisterPrompt<'a> {
    pub title: &'a str,
}

#[get("/register")]
pub async fn display_register_page() -> Result<HttpResponse, WebsiteError> {
    let template = RegisterPrompt::new("Register");
    let body = template.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}
