use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "todos"]
struct Todo {
    id: i32,
    title: String,
    content: String
}