use std::sync::Arc;

use actix_web::{web::Data, App, HttpServer};
mod actix_scope;
use actix_scope::generated_scope;
use sqlx::postgres::PgPoolOptions;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    /*
    let pool = PgPoolOptions::new()
        .max_connections(std::env::)
        .connect("")
        .await
        .unwrap();

    let state = AppState {
        pool
    };
    */

    HttpServer::new(move || App::new().service(generated_scope()))
        .bind(("0.0.0.0", 8081))?
        .run()
        .await
}

#[derive(Clone, Debug)]
pub struct AppState {
    pub pool: sqlx::postgres::PgPool,
}
