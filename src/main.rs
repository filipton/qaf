use actix_web::{web::Data, App, HttpServer};
mod actix_scope;
use actix_scope::generated_scope;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    let options = StartupOptions::new();

    let pool = PgPoolOptions::new()
        .max_connections(options.max_connections)
        .connect(&options.connection_string)
        .await
        .unwrap();

    let state = AppState { pool };

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(state.clone()))
            .service(generated_scope())
    })
    .bind(options.bind_address)?
    .run()
    .await
}

#[derive(Clone, Debug)]
pub struct AppState {
    pub pool: sqlx::postgres::PgPool,
}

pub struct StartupOptions {
    pub connection_string: String,
    pub max_connections: u32,

    pub bind_address: String,
}

impl StartupOptions {
    pub fn new() -> Self {
        Self {
            connection_string: std::env::var("DATABASE_URL")
                .expect("You must set DATABASE_URL env variable!"),

            max_connections: std::env::var("MAX_CONNECTIONS")
                .unwrap_or("5".to_string())
                .parse()
                .unwrap(),

            bind_address: std::env::var("BIND_ADDRESS").unwrap_or("127.0.0.1:8080".to_string()),
        }
    }
}
