use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RequestBodyCreateTodo {
    pub title: String,
    pub description: Option<String>,
    pub date_deadline: Option<NaiveDateTime>,
}
