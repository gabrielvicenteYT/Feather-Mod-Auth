use uuid::Uuid;
use crate::models::DbDateTime;
use sqlx::PgPool;
use crate::utils::error::WebsiteError;
use crate::utils::password_hashing::hash_password;
use log::info;
use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Deserialize)]
pub struct UserCreationRequest {
    username: String,
    first_name: String,
    last_name: String,
    password: String,
}

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct User {
    id: Uuid,
    username: String,
    first_name: String,
    last_name: String,
    #[serde(skip_serializing)]
    password: Vec<u8>,
    creation_date: DbDateTime,
    update_date: DbDateTime,
}

impl User {
    pub async fn create_user(req: UserCreationRequest, pool: &PgPool) -> Result<(), WebsiteError> {
        let hash = hash_password(req.password)?;
        let mut tx = pool.begin().await?;
        let result = sqlx::query(r#"
            INSERT INTO minos.users (username, first_name, last_name, password) VALUES ($1, $2, $3, $4)
        "#
        )
            .bind(req.username)
            .bind(req.first_name)
            .bind(req.last_name)
            .bind(&hash)
            .execute(&mut tx)
            .await?;

        tx.commit().await?;
        info!("Database result: {:#?}", result);
        Ok(())
    }
}