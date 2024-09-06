use std::{env, sync::Mutex};

use aayojak_server::{
    db::postgres_connection,
    services::{
        base::{self},
        service_todo::{create_todo::create_todo, read_todo::get_all_todos},
    },
    structures::app_state::AppState,
};
use actix_web::{middleware, web, App, HttpServer};
use dotenvy::dotenv;

// MAIN SERVER
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let pg_url = env::var("DATABASE_URL").expect("DATABASE_URL not found [REQUIRED]");
    let pg_connection = postgres_connection::establish_postgres_connection(&pg_url);
    let pg_connection_webdata = web::Data::new(AppState {
        pg_connection: Mutex::new(pg_connection),
    });

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::NormalizePath::new(
                middleware::TrailingSlash::Trim,
            ))
            .service(base::welcome)
            .service(base::echo)
            .app_data(pg_connection_webdata.clone())
            .service(
                web::scope("/api")
                    .service(create_todo)
                    .service(get_all_todos),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
