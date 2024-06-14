use std::{collections::HashMap, num::ParseIntError, sync::Mutex};

use aayojak_lib::Todo;
use actix_web::{delete, get, post, web, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;

use crate::services::base::AppState;

// Structures used only for server-side processing of data
pub struct TodoList {
    pub map: HashMap<i32, Todo>,
    pub id_counter: i32,
}

#[derive(Deserialize)]
struct CreateTodoBody {
    title: String,
}

/// Create todo
#[post("/todo/create")]
pub async fn create_todo(
    todo_request_body: web::Json<CreateTodoBody>,
    data: web::Data<AppState>,
) -> impl Responder {
    let app_state_result = data.todo_list.lock();
    if let Err(err) = app_state_result {
        return HttpResponse::InternalServerError()
            .body(format!("Error acquiring mutex-guard: {err}"));
    }

    let mut app_state = app_state_result.unwrap();

    let new_id = app_state.id_counter;
    // create todo
    println!("Creating todo...");
    let todo = Todo::new(&todo_request_body.title, new_id);
    app_state.map.insert(new_id, todo);
    let inserted_todo = app_state.map.get(&new_id).unwrap();

    //parse response
    match serde_json::to_string(inserted_todo) {
        Ok(response_body_todo) => {
            app_state.id_counter += 1;
            return HttpResponse::Ok().body(response_body_todo);
        }
        Err(err) => {
            app_state.id_counter += 1;
            return HttpResponse::InternalServerError()
                .body(format!("Error parsing response body: {err}"));
        }
    }
}

/// Get all todos
#[get("/todo/")]
pub async fn get_all_todos(data: web::Data<AppState>) -> impl Responder {
    let app_state = data.todo_list.lock().unwrap();
    HttpResponse::Ok().body(serde_json::to_string(&app_state.map).unwrap())
}

/// Get a todo by id
#[get("/todo/{id}")]
pub async fn get_todo_by_id(data: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    let app_state = data.todo_list.lock().unwrap();
    let id_string = req.match_info().get("id").unwrap();

    let id: Result<i32, ParseIntError> = id_string.parse();

    match id {
        Ok(id_int) => {
            if let Some(todo) = app_state.map.get(&id_int) {
                return HttpResponse::Ok().body(serde_json::to_string(todo).unwrap());
            } else {
                return HttpResponse::NotFound().body(format!("Todo for id {id_int} not found"));
            }
        }
        Err(err) => {
            return HttpResponse::BadRequest().body(format!(
                "Could not parse {id_string} into an integer: {err}"
            ));
        }
    }
}

/// Remove a todo by id
#[delete("/todo/{id}")]
pub async fn delete_todo_by_id(data: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    let mut app_state = data.todo_list.lock().unwrap();
    let id_string = req.match_info().get("id").unwrap();

    let id: Result<i32, ParseIntError> = id_string.parse();

    match id {
        Ok(id_int) => {
            if let Some(todo) = app_state.map.remove(&id_int) {
                return HttpResponse::Ok().body(serde_json::to_string(&todo).unwrap());
            } else {
                return HttpResponse::NotFound().body(format!("Todo for id {id_int} not found"));
            }
        }
        Err(err) => {
            return HttpResponse::BadRequest().body(format!(
                "Could not parse {id_string} into an integer: {err}"
            ));
        }
    }
}
