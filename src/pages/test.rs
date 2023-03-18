use actix_web::{get, post, HttpResponse, Responder};

#[get("/")]
pub async fn get_test_index() -> impl Responder {
    return HttpResponse::Ok().body("Hello wordlcxz!");
}

#[actix_web::post("/")]
pub async fn get_post_index() -> impl Responder {
    return HttpResponse::Ok().body("Hello wordleqw!");
}

pub async fn get_post_indedsadsadx() -> impl Responder {
    return HttpResponse::Ok().body("Hello wordl!");
}
