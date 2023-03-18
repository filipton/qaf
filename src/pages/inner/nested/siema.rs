use actix_web::{get, HttpResponse, Responder};

#[get("")]
pub async fn get_kurwa_index() -> impl Responder {
    return HttpResponse::Ok().body("Hello WITAJ world!!!!!");
}
