use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Visit::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Visit::Ip).string().not_null())
                    .col(ColumnDef::new(Visit::Name).string().not_null())
                    .col(ColumnDef::new(Visit::VisitedAt).timestamp_with_time_zone().not_null())
                    .primary_key(Index::create().col(Visit::Ip).col(Visit::VisitedAt))
                    .to_owned(),
            )
            .await?;

        // Create indexes
        manager
            .create_index(
                Index::create()
                    .table(Visit::Table)
                    .name("idx_visit_visited_at")
                    .col(Visit::VisitedAt)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .table(Visit::Table)
                    .name("idx_visit_ip")
                    .col(Visit::Ip)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Visit::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Visit {
    Table,
    Ip,
    Name,
    VisitedAt,
}

