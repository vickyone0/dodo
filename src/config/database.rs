
use serde::Deserialize;
use sqlx::postgres::{PgPool, PgPoolOptions};


#[derive(Debug, Deserialize)]
pub struct DatabaseConfig{
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub dbname: String

}

impl DatabaseConfig{
    pub fn from_env() -> Result<Self, config::ConfigError>{
        config::Config::builder()
        .add_source(config::Environment::default())
        .build()?
        .try_deserialize()
    }

    pub fn connection_string(&self) -> String{
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user, self.password, self.host, self.port, self.dbname
        )
    }

    pub async fn create_pool(&self) -> Result<PgPool, sqlx::Error> {
        PgPoolOptions::new()
        .max_connections(5)
        .connect(&self.connection_string())
        .await

    }
}
