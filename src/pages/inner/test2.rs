use actix_web::{get, HttpResponse, Responder};

#[get("test")]
pub async fn get_test2_index() -> impl Responder {
    return HttpResponse::Ok().body("Hello INNERworld!!!!!");
}
