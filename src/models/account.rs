use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Account{
    pub id:Uuid,
    pub user_id:Uuid,
    pub account_type:String,
    pub account_number:String,
    pub balance:i64,
    pub currency: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

//Account creation DTO
#[derive(Debug, Deserialize, Validate)]
pub struct CreateAccount {
    #[validate(length(min = 1))]
    pub account_type: String,

    #[validate(length(min = 1))]
    pub currency: String,
}


//account response DTO
#[derive(Debug, Serialize)]
pub struct AccountResponse {
    pub id: Uuid,
    pub account_type: String,
    pub account_number: String,
    pub balance: f64,
    pub currency: String,
}