use std::num::ParseIntError;
use std::ops::DerefMut;

use crate::structures::todos::rb_update_todo::RequestBodyUpdateTodo;
use crate::structures::todos::todo::Todo;
use actix_web::{post, web, HttpRequest, HttpResponse, Responder};
use diesel::associations::HasTable;
use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};

use crate::structures::app_state::AppState;

use crate::schema::todos::dsl::*;

/// Update todo
#[post("/todo/update/{id}")]
pub async fn update_todo(
    todo_request: web::Json<RequestBodyUpdateTodo>,
    data: web::Data<AppState>,
    req: HttpRequest,
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
            println!("Updating todo...");
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
            let todo_request_body = todo_request.0;

            let existing_todo_result = todos
                .find(id_int)
                .select(Todo::as_select())
                .first(pg_connection);

            if let Err(err) = existing_todo_result {
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

            let mut existing_todo = existing_todo_result.unwrap();
            let mut description_allow_none_overwrite = false;
            let mut date_deadline_allow_none_overwrite = false;

            if todo_request_body.title.is_some() {
                existing_todo.set_title(todo_request_body.title.unwrap());
            }

            if todo_request_body.completion_status.is_some() {
                existing_todo
                    .set_completion_status(todo_request_body.completion_status.unwrap(), true);
            }

            if todo_request_body.description_ano.is_some() {
                description_allow_none_overwrite = todo_request_body.description_ano.unwrap();
            }

            if todo_request_body.date_deadline_ano.is_some() {
                date_deadline_allow_none_overwrite = todo_request_body.date_deadline_ano.unwrap();
            }

            match todo_request_body.description {
                Some(todo_des) => existing_todo.set_description(Some(&todo_des[..])),
                None => {
                    if description_allow_none_overwrite {
                        existing_todo.set_description(None)
                    }
                }
            }

            match todo_request_body.date_deadline {
                Some(todo_date_deadline) => {
                    existing_todo.set_date_deadline(Some(todo_date_deadline))
                }
                None => {
                    if date_deadline_allow_none_overwrite {
                        existing_todo.set_date_deadline(None)
                    }
                }
            }

            let updated_todo_result = diesel::update(todos::table())
                .set(existing_todo)
                .get_result::<Todo>(pg_connection);

            match updated_todo_result {
                // parse result into string for the response
                Ok(updated_todo) => match serde_json::to_string(&updated_todo) {
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
