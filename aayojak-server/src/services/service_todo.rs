use std::{collections::HashMap, sync::Mutex};

use aayojak_lib::Todo;
use actix_web::{get, post, web, HttpResponse, Responder};
use serde::Deserialize;

// Structures used only for server-side processing of data
pub struct TodoList {
    pub map: HashMap<i32, Todo>,
    pub id_counter: i32,
}

pub struct AppState {
    pub todo_list: Mutex<TodoList>,
}

#[derive(Deserialize)]
struct CreateTodoBody {
    title: String,
}

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

#[get("/todo/get")]
pub async fn get_all_todos(data: web::Data<AppState>) -> impl Responder {
    let app_state = data.todo_list.lock().unwrap();
    HttpResponse::Ok().body(serde_json::to_string(&app_state.map).unwrap())
}
