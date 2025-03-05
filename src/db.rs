use mongodb::{Client, Database, Collection};
use mongodb::options::ClientOptions;
use dotenvy::dotenv;
use std::env;
use crate::models::Todo;

pub async fn init_db() -> Database {
    dotenv().ok();
    let uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set in .env");
    let db_name = env::var("DATABASE_NAME").expect("DATABASE_NAME must be set in .env");

    let mut client_options = ClientOptions::parse(&uri).await.expect("Failed to parse options");

    client_options.app_name = Some("RustMongoAPI".to_string());

    let client = Client::with_options(client_options).expect("Failed to initialize client");
    client.database(&db_name)
}

pub fn todos_collection(db: &Database) -> Collection<Todo> {
    db.collection::<Todo>("todos")
}
