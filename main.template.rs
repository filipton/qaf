use actix_web::{App, HttpServer};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
        //SERVICES
    )
        .bind(("0.0.0.0", 8081))?
        .run()
        .await
}

//MOD_PAGES
