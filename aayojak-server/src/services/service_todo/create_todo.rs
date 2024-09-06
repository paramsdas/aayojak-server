use std::ops::DerefMut;

use crate::structures::todo::Todo;
use actix_web::{post, web, HttpResponse, Responder};
use diesel::dsl::insert_into;
use diesel::RunQueryDsl;

use crate::structures::app_state::AppState;

use crate::schema::todos::dsl::*;

/// Create todo
#[post("/todo/create")]
pub async fn create_todo(
    todo_request: web::Json<Todo>,
    data: web::Data<AppState>,
) -> impl Responder {
    let app_state_result = data.pg_connection.lock();

    match app_state_result {
        // handle error, early fail
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Error acquiring mutex-guard: {err}"))
        }
        // create todo
        Ok(mut pg_connection_guard) => {
            let pg_connection = pg_connection_guard.deref_mut();
            println!("Creating todo...");
            let todo = todo_request.0;
            let inserted_todo_result = insert_into(todos)
                .values(&todo)
                .get_result::<Todo>(pg_connection);

            match inserted_todo_result {
                // parse result into string for the response
                Ok(inserted_todo) => match serde_json::to_string(&inserted_todo) {
                    Ok(response_body_todo) => HttpResponse::Ok().body(response_body_todo),
                    Err(err) => HttpResponse::InternalServerError()
                        .body(format!("Error parsing response body: {err}")),
                },
                Err(err) => HttpResponse::ExpectationFailed().body(format!(
                    "Something went wrong while inserting the new todo: {err}"
                )),
            }
        }
    }
}
