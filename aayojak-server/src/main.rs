use std::{collections::HashMap, sync::Mutex};

use aayojak_server::services::{
    base::{self, AppState},
    service_todo::{self, TodoList},
};
use actix_web::{web, App, HttpServer};

// MAIN SERVER
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let todo_list = web::Data::new(AppState {
        todo_list: Mutex::new(TodoList {
            map: HashMap::new(),
            id_counter: 0,
        }),
    });

    HttpServer::new(move || {
        App::new()
            .service(base::welcome)
            .service(base::echo)
            .app_data(todo_list.clone())
            .service(
                web::scope("/api")
                    .service(service_todo::create_todo)
                    .service(base::api_version)
                    .service(service_todo::get_all_todos),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
