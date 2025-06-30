use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Todo {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub title: String,
    pub description: String,
    pub due_date: DateTime,
    pub completed: bool,
}

#[derive(Debug, Deserialize)]
pub struct NewTodo {
    pub title: String,
    pub description: String,
    pub due_date: DateTime,
}
