use sea_orm_migration::prelude::*;

mod m20220101_000001_create_users_table;

#[async_std::main]
async fn main() {
    cli::run_cli(migration::Migrator).await;
}
