use actix_web::{delete, web, HttpRequest, HttpResponse, Responder};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use std::num::ParseIntError;
use std::ops::DerefMut;

use crate::structures::app_state::AppState;

use crate::schema::todos::dsl::*;

/// Remove a todo by id
#[delete("/todo/{id}")]
pub async fn delete_todo_by_id(data: web::Data<AppState>, req: HttpRequest) -> impl Responder {
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

    let deleted_todo_result = diesel::delete(todos.filter(id.eq(id_int))).execute(pg_connection);

    if let Err(err) = deleted_todo_result {
        return HttpResponse::InternalServerError()
            .body(format!("could not delete todo with id {id_int}: {err}"));
    }

    match serde_json::to_string(&deleted_todo_result.unwrap()) {
        Err(err) => {
            return HttpResponse::InternalServerError()
                .body(format!("could not delete todo with id {id_int}: {err}"));
        }
        Ok(response_body) => {
            return HttpResponse::Ok().body(response_body);
        }
    }
}
