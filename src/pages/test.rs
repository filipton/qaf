use actix_web::{get, post, web::Data, HttpResponse, Responder};

use crate::AppState;

#[get("/")]
pub async fn get_test_index(data: Data<AppState>) -> impl Responder {
    return HttpResponse::Ok().body(format!("Hello {}!", "1"));
}

#[actix_web::post("/")]
pub async fn get_post_index() -> impl Responder {
    return HttpResponse::Ok().body("Hello wordleqw!");
}

pub async fn get_post_indedsadsadx() -> impl Responder {
    return HttpResponse::Ok().body("Hello wordl!");
}
