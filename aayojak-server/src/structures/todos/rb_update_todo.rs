use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RequestBodyUpdateTodo {
    pub title: Option<String>,
    pub description: Option<String>,
    pub description_ano: Option<bool>, // ano -> allow 'None' overwrite for description
    pub date_deadline: Option<NaiveDateTime>,
    pub date_deadline_ano: Option<bool>, // ano -> allow 'None' overwrite for date_deadline
    pub completion_status: Option<bool>,
}
