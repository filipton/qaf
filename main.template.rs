use actix_web::{App, HttpServer};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(pages::test::get_test_index))
        .bind(("0.0.0.0", 8081))?
        .run()
        .await
}

//MOD_PAGES
