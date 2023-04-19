use actix_web::{get, post, web::Data, HttpResponse, Responder};

use crate::AppState;

#[derive(Debug, sqlx::FromRow)]
pub struct Test {
    pub id: i32,
    pub value: i32,
}

#[get("/")]
pub async fn get_tests(data: Data<AppState>) -> impl Responder {
    let mut conn = data.pool.acquire().await.unwrap();

    let rows = sqlx::query_as!(Test, "SELECT * FROM tests")
        .fetch_all(&mut conn)
        .await
        .unwrap();

    return HttpResponse::Ok().body(format!("Hello {:?}!", rows));
}

#[actix_web::post("/")]
pub async fn add_test(data: Data<AppState>) -> impl Responder {
    let mut conn = data.pool.acquire().await.unwrap();

    let id = rand::random::<i32>();

    sqlx::query!("INSERT INTO tests (value) VALUES (?)", id)
        .execute(&mut conn)
        .await
        .unwrap();

    return HttpResponse::Ok().body(format!("Hello!"));
}

// THIS WONT BE ADDED TO THE ACTIX SCOPE
pub async fn get_post_indedsadsadx() -> impl Responder {
    return HttpResponse::Ok().body("Hello wordl!");
}
