use sea_orm_migration::{manager, prelude::*};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000001_create_chat"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Chat::Table)
                    .col(
                        ColumnDef::new(Chat::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Chat::TopicId).integer().not_null())
                    .col(ColumnDef::new(Chat::UserId).integer().not_null())
                    .col(
                        ColumnDef::new(Chat::Timestamp)
                            .date_time()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_string())
                            .not_null(),
                    )
                    .col(ColumnDef::new(Chat::Message).text())
                    .col(ColumnDef::new(Chat::Image).string())
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx-chat-topic-time")
                    .table(Chat::Table)
                    .col(Chat::TopicId)
                    .col(Chat::Timestamp)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Chat::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Chat {
    Table,
    Id,
    TopicId,
    UserId,
    Timestamp,
    Message,
    Image,
}
