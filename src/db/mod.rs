pub mod users;

use std::env;

use tokio::sync::OnceCell;

use sea_orm::*;

static DB: OnceCell<DatabaseConnection> = OnceCell::const_new();

pub async fn init_db() {
    let db_url = env::var("DATABASE_URL")
        .expect("Failed to load DATABASE_URL. Ensure variable DATABASE_URL exist in .env");

    let connection = Database::connect(db_url)
        .await
        .expect("Failed connecting to the database");

    DB.set(connection).expect("Failed to connect to db");
}

pub fn get_db() -> &'static DatabaseConnection {
    DB.get().expect("Db is not initialized")
}
