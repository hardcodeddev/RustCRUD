use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use mongodb::bson::{doc, extjson::de::Error};
use mongodb::results::InsertOneResult;
use mongodb::bson::oid::ObjectId;
use crate::{db, models::{Todo, NewTodo}};
use std::{collections, sync::Arc};
use thiserror::Error;
use serde_json::json;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("MongoDB error: {0}")]
    MongoError(#[from] mongodb::error::Error),
    #[error("Todo not found")]
    NotFound,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let code = match self {
            ApiError::MongoError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::NotFound => StatusCode::NOT_FOUND
        };
        let body = Json(json!({"error": self.to_string() }));
        (code, body).into_response()
    }
}

pub type AppState = Arc<mongodb::Database>;

pub async fn create_todo(
    State(db): State<AppState>,
    Json(payload): Json<NewTodo>,
) -> Result<impl IntoResponse, ApiError> {
    let collection = db::todos_collection(&db);
    let mut todo = Todo {
        id: None,
        title: payload.title,
        description: payload.description,
        due_date: payload.due_date,
        completed: false
    };

    let uuid = uuid::Uuid::new_v4();
    todo.id = Some(uuid.to_string());

    let insert_result: InsertOneResult = collection.insert_one(todo.clone(), None).await?;

    Ok((StatusCode::CREATED, Json(todo)))
}

pub async fn get_todo(
    State(db): State<AppState>,
    Path(id): Path<String>
) -> Result<impl IntoResponse, ApiError> {
    let collection = db::todos_collection(&db);

    let filter = doc! {"id": &id};
    if let Some(todo) = collection.find_one(filter, None).await? {
        Ok(Json(todo))
    } else {
        Err(ApiError::NotFound)
    }
}