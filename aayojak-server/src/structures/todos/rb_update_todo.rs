use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RequestBodyUpdateTodo {
    pub title: Option<String>,
    pub description: Option<String>,
    pub date_deadline: Option<NaiveDateTime>,
    pub completion_status: Option<bool>,
}
