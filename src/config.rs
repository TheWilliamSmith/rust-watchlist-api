use sea_orm::{Database, DatabaseConnection, DbErr};
use std::env;

pub async fn connect_database() -> Result<DatabaseConnection, DbErr> {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");

    Database::connect(&database_url).await
}