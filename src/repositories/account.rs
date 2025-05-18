use sqlx::{PgPool, postgres::PgConnection};
use uuid::Uuid;
use crate::{models::account::Account, errors::http_error::HttpError};

#[derive(Debug)]
pub struct AccountRepository {
    pool: PgPool,
}

impl AccountRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Account, HttpError> {
        sqlx::query_as!(
            Account,
            r#"SELECT * FROM accounts WHERE id = $1"#,
            id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(Into::into)
    }

    pub async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<Account>, HttpError> {
        sqlx::query_as!(
            Account,
            r#"SELECT * FROM accounts WHERE user_id = $1"#,
            user_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(Into::into)
    }

    pub async fn update_balance(
        &self,
        transaction: &mut PgConnection,
        account_id: Uuid,
        amount: i64
    ) -> Result<(), HttpError> {
        sqlx::query!(
            r#"UPDATE accounts 
               SET balance = balance + $1, 
                   updated_at = NOW() 
               WHERE id = $2"#,
            amount,
            account_id
        )
        .execute(&mut *transaction)
        .await?;
        Ok(())
    }
}