use actix_web::{get, post, HttpResponse, Responder};

// Base endpoints
#[get("/")]
pub async fn welcome() -> impl Responder {
    HttpResponse::Ok().body("Welcome to aayojak!")
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
