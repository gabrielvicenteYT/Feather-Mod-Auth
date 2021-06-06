use crate::utils::error::WebsiteError;
use actix_web::{get, HttpResponse};
use askama::Template;
use derive_more::Constructor;

#[derive(Template, Constructor)]
#[template(path = "index.html")]
pub struct Index<'a> {
    pub title: &'a str,
}

#[get("/home")]
pub async fn display_home_page() -> Result<HttpResponse, WebsiteError> {
    let template = Index::new("Home");
    let body = template.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}
