use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Items::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Items::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Items::Title).string().not_null().unique_key())
                    .col(ColumnDef::new(Items::Kind).string().not_null())
                    .col(ColumnDef::new(Items::Status).string().not_null())
                    .col(ColumnDef::new(Items::ReleaseDate).date().not_null())
                    .col(ColumnDef::new(Items::Rating).float().not_null())
                    .col(ColumnDef::new(Items::Notes).string().not_null())
                    .col(ColumnDef::new(Items::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Items::UpdatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .drop_table(Table::drop().table(Items::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Items {
    Table,
    Id,
    Title,
    Kind,
    Status,
    ReleaseDate,
    Rating,
    Notes,
    CreatedAt,
    UpdatedAt,
}
