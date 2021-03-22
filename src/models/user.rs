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
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
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
        let result = sqlx::query!(r#"
            INSERT INTO minos.users (username, first_name, last_name, password) VALUES ($1, $2, $3, $4)
        "#,
           req.username,
           req.first_name,
           req.last_name,
            &hash
        )
            .execute(&mut tx)
            .await?;

        tx.commit().await?;
        info!("Database result: {:#?}", result);
        Ok(())
    }

    pub async fn check_username_availability(username: &String, pool: &PgPool) -> Result<bool, WebsiteError>{
        let mut tx = pool.begin().await?;
        let result = sqlx::query(r#"
            SELECT 1 FROM minos.users WHERE username = $1
        "#)
            .bind(username)
            .fetch_optional(&mut tx)
            .await?;
        Ok(result.is_none())
    }
}

impl UserCreationRequest {
    pub fn verify_input(&mut self) {
        // Convert username to lower
        self.username = self.username.to_lowercase()
        // TODO: Set up password policy
    }
}