use actix_web::{get, HttpResponse, Responder};

#[get("/")]
pub async fn get_test_index() -> impl Responder {
    return HttpResponse::Ok().body("Hello wordl!");
}

#[post("/")]
pub async fn get_post_index() -> impl Responder {
    return HttpResponse::Ok().body("Hello wordl!");
}
