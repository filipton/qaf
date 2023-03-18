use actix_web::{App, HttpServer};
mod actix_scope;
use actix_scope::generated_scope;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(generated_scope()))
        .bind(("0.0.0.0", 8081))?
        .run()
        .await
}
