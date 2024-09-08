use crate::structures::todos::todo::Todo;
use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
use std::num::ParseIntError;
use std::ops::DerefMut;

use crate::structures::app_state::AppState;

use crate::schema::todos::dsl::*;

/// Get all todos
#[get("/todo")]
pub async fn get_all_todos(data: web::Data<AppState>) -> impl Responder {
    let app_state_result = data.pg_connection.lock();

    if let Err(err) = app_state_result {
        return HttpResponse::InternalServerError()
            .body(format!("Error acquiring mutex-guard: {err}"));
    }

    let mut pg_connection_guard = app_state_result.unwrap();
    let pg_connection = pg_connection_guard.deref_mut();
    // get all todos
    let todos_result = todos.select(Todo::as_select()).load::<Todo>(pg_connection);

    if let Err(err) = todos_result {
        return HttpResponse::ExpectationFailed().body(format!(
            "Unexpected error while inserting the new todo: {err}"
        ));
    }

    let todo_list = todos_result.unwrap();

    match serde_json::to_string(&todo_list) {
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Error parsing response body: {err}"))
        }
        Ok(response) => HttpResponse::Ok().body(response),
    }
}

/// Get a todo by id
#[get("/todo/{id}")]
pub async fn get_todo_by_id(data: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    let app_state_result = data.pg_connection.lock();

    if let Err(err) = app_state_result {
        return HttpResponse::InternalServerError()
            .body(format!("Error acquiring mutex-guard: {err}"));
    }

    let mut pg_connection_guard = app_state_result.unwrap();
    let pg_connection = pg_connection_guard.deref_mut();
    let id_string_option = req.match_info().get("id");
    if let None = id_string_option {
        return HttpResponse::InternalServerError()
            .body("Could not get todo 'id' from request url");
    }

    let id_result: Result<i64, ParseIntError> = id_string_option.unwrap().parse();
    if let Err(err) = id_result {
        return HttpResponse::BadRequest().body(format!(
            "todo 'id' must be a number but could not be parsed: {err}"
        ));
    }
    let id_int = id_result.unwrap();

    let todo_result = todos
        .find(id_int)
        .select(Todo::as_select())
        .first(pg_connection);
    if let Err(err) = todo_result {
        match err {
            diesel::result::Error::NotFound => {
                return HttpResponse::NotFound()
                    .body(format!("Could not find todo for id {id_int}"));
            }
            other => {
                return HttpResponse::InternalServerError().body(format!(
                    "unexpected error while fetching todo with id {id_int}: {other}"
                ));
            }
        }
    }
    let todo = todo_result.unwrap();
    match serde_json::to_string(&todo) {
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Error parsing response body: {err}"))
        }
        Ok(response) => HttpResponse::Ok().body(response),
    }
}
