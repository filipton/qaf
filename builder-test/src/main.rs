use actix_web::{web::Data, App, HttpServer};
mod actix_scope;
use actix_scope::generated_scope;
use fn_stack::{AppState, StartupOptions};
use sqlx::mysql::MySqlPoolOptions;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    let options = StartupOptions::new();

    let pool = MySqlPoolOptions::new()
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
