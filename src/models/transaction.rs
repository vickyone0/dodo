use serde::{Deserialize, Serialize};
use sqlx::Type;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Transaction {
    pub id: Uuid,
    pub from_account: Uuid,
    pub to_account: Uuid,
    pub amount: i64,
    pub status: TransactionStatus,
    pub created_at: chrono::DateTime<chrono::Utc>,
}


#[derive(Debug, Serialize, Deserialize, Type, PartialEq)]
#[sqlx(type_name = "transaction_status", rename_all = "lowercase")]
pub enum TransactionStatus {
    Pending,
    Completed,
    Failed,
}
