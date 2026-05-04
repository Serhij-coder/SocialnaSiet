use sea_orm_migration::{manager, prelude::*};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000001_create_posts"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Posts::Table)
                    .col(
                        ColumnDef::new(Posts::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Posts::TopicId).integer().not_null())
                    .col(ColumnDef::new(Posts::UserId).integer().not_null())
                    .col(ColumnDef::new(Posts::Header).string().not_null())
                    .col(ColumnDef::new(Posts::Content).text())
                    .col(ColumnDef::new(Posts::Image).string())
                    .col(
                        ColumnDef::new(Posts::Timestamp)
                            .date_time()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_string())
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx-posts-topic-time")
                    .table(Posts::Table)
                    .col(Posts::TopicId)
                    .col(Posts::Timestamp)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Posts::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Posts {
    Table,
    Id,
    TopicId,
    UserId,
    Header,
    Content,
    Image,
    Timestamp,
}