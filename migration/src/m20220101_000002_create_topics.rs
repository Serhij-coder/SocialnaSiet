use sea_orm_migration::{manager, prelude::*};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000001_create_topics"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Topics::Table)
                    .col(
                        ColumnDef::new(Topics::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Topics::Name)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Topics::NoSpacesName).string().not_null())
                    .col(ColumnDef::new(Topics::TopicPicture).string())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Topics::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Topics {
    Table,
    Id,
    Name,
    NoSpacesName,
    TopicPicture,
}
