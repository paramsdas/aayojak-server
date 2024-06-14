use std::sync::Mutex;

use actix_web::{get, post, HttpResponse, Responder};

use super::service_todo::TodoList;

pub struct AppState {
    pub todo_list: Mutex<TodoList>,
}

// Base endpoints
#[get("/")]
pub async fn welcome() -> impl Responder {
    HttpResponse::Ok().body("Hello world, welcome to aayojak!")
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("/")]
pub async fn api_version() -> impl Responder {
    HttpResponse::Ok().body("v0.0.1")
}
