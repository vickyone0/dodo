
use actix_web::{middleware, web, App, HttpServer};
use sqlx::postgres::PgPool;
use sqlx;

use config::DatabaseConfig;

mod config;
mod models;
mod schemas;
mod services;
mod repositories;
mod routes;
mod errors;
mod middleware;


#[actix_web::main]
async fn main() -> std::io::Result<()>{


    dotenv::dotenv().ok();

    //load configartion 
    let db_config = DatabaseConfig::from_env().expect("Failed to load config");
    
    //setup db connectionn pool
    let db_pool=  PgPool::connect(&db_config.connection_string())
    .await
    .expect("Failed to create pool");

    //run migration
    sqlx::migrate!().run(&db_pool).await.expect("Failed to run migrations");

    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(db_pool.clone()))
        .configure(route::config)
        .wrap(middleware::Logger::default())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}