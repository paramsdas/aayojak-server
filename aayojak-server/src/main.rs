use std::{env, sync::Mutex};

use aayojak_server::{
    db::postgres_connection,
    services::{
        base::{self},
        service_todo::{create_todo::create_todo, read_todo::get_all_todos},
    },
    structures::app_state::AppState,
};
use actix_web::{error, middleware, web, App, HttpResponse, HttpServer};

// MAIN SERVER
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let host = env::var("HOST_ADDRESS").unwrap_or_else(|_| {
        println!("HOST_ADDRESS env not set, using 127.0.0.1");
        "127.0.0.1".to_string()
    });
    let port = 8080;
    println!("getting pg_url");
    let pg_url = env::var("DATABASE_URL").expect("DATABASE_URL not found [REQUIRED]");
    println!("connecting to database");
    let pg_connection = postgres_connection::establish_postgres_connection(&pg_url);
    let pg_connection_webdata = web::Data::new(AppState {
        pg_connection: Mutex::new(pg_connection),
    });

    println!("Listening on host {host} and port {port} ...");

    HttpServer::new(move || {
        let json_config = web::JsonConfig::default()
            .limit(4096)
            .error_handler(|err, _req| {
                // create custom error response
                error::InternalError::from_response(
                    "",
                    HttpResponse::BadRequest()
                        .content_type("application/json")
                        .body(format!(r#"{{"error":"{}"}}"#, err)),
                )
                .into()
            });
        App::new()
            .wrap(middleware::NormalizePath::new(
                middleware::TrailingSlash::Trim,
            ))
            .service(base::welcome)
            .service(base::echo)
            .app_data(json_config)
            .app_data(pg_connection_webdata.clone())
            .service(
                web::scope("/api")
                    .service(create_todo)
                    .service(get_all_todos),
            )
    })
    .bind((host, 8080))?
    .run()
    .await
}
