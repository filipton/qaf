use actix_web::{Responder, HttpResponse, get};


#[get("/")]
pub async fn get_test_index() -> impl Responder {
    return HttpResponse::Ok().body("Hello wordl!");
}
