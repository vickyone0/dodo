use sqlx::{PgPool, PgConnection, FromRow};
use uuid::Uuid;
use crate::{
    models::transaction::{Transaction, TransactionStatus},
    errors::http_error::HttpError
};

#[derive(Debug)]
pub struct TransactionRepository {
    pool: PgPool,
}

impl TransactionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn begin(&self) -> Result<PgConnection, HttpError> {
        self.pool.begin().await.map_err(Into::into)
    }

    pub async fn create(
        &self,
        transaction: &mut PgConnection,
        from_account: Uuid,
        to_account: Uuid,
        amount: i64,
        status: TransactionStatus
    ) -> Result<Transaction, HttpError> {
        sqlx::query_as!(
            Transaction,
            r#"INSERT INTO transactions 
               (from_account, to_account, amount, status) 
               VALUES ($1, $2, $3, $4) 
               RETURNING *"#,
            from_account,
            to_account,
            amount,
            status as _
        )
        .fetch_one(&mut *transaction)
        .await
        .map_err(Into::into)
    }

    pub async fn find_by_account(
        &self,
        account_id: Uuid
    ) -> Result<Vec<Transaction>, HttpError> {
        sqlx::query_as!(
            Transaction,
            r#"SELECT * FROM transactions 
               WHERE from_account = $1 OR to_account = $1"#,
            account_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(Into::into)
    }
}