use actix_web::{get, HttpResponse, Result};

#[get("/")]
pub async fn home_page() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().content_type("application/json").body(
        r#"
        {
            "name": "minos-auth",
            "version": "0.0.1",
            "documentation": "Soon:tm:",
            "about": "Welcome traveler!"
        }
        "#,
    ))
}
