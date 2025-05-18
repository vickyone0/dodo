use argon2::password_hash;
use sqlx::{PgPool, FromRow};
use uuid::Uuid;
use crate::{models::user::User, errors::HttpError};

#[derive(Debug)]
pub struct UserRepository{
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self{ pool}
    }

    pub async fn find_by_id(&self, id:Uuid) -> Result<User, HttpError> {
        sqlx::query_as!(
            User,
            r#"SELECT * FROM users WHERE id = $1"#,
            id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(Into::into)
    }

    pub async fn find_by_email(&self, email:&str) -> Result<User, HttpError> {
        sqlx::query_as!(
            User,
            r#"SELECT * FROM users WHERE email = $1"#,
            email
        )
        .fetch_one(&self.pool)
        .await
        .map_err(Into::into)
    }

    pub async fn create_user(
        &self,
        username:&str,
        email: &str,
        password_hash: &str
    ) -> Result<User, HttpError> {
        sqlx::query_as!(
            User,
            r#"INSERT INTO users (username, email, password_has)
            VALUES ($1, $2, $3)
            RETURNING *"#,
            username,
            email,
            password_hash
        )
        .fetch_one(&self.pool)
        .await
        .map_err(Into::into)
    }
}