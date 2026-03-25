pub mod users;

use tokio::sync::OnceCell;

use sea_orm::*;

static DB: OnceCell<DatabaseConnection> = OnceCell::const_new();

pub async fn init_db() {
    let connection = Database::connect("postgresql://serhii@localhost:5432/postgres")
        .await
        .expect("Failed connecting to the database");

    DB.set(connection).expect("Failed to connect to db");
}

pub fn get_db() -> &'static DatabaseConnection {
    DB.get().expect("Db is not initialized")
}
