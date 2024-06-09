use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .alter_table(
                Table::alter()
                    .table(User::Table)
                    .modify_column(
                        ColumnDef::new(User::Username)
                            .string()
                            .not_null()
                            .unique_key()
                    )
                    .modify_column(
                        ColumnDef::new(User::Email)
                            .string()
                            .not_null()
                            .unique_key()
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .alter_table(
                Table::alter()
                    .table(User::Table)
                    .modify_column(
                        ColumnDef::new(User::Username)
                            .string()
                            .not_null()
                    )
                    .modify_column(
                        ColumnDef::new(User::Email)
                            .string()
                            .not_null()
                    )
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Username,
    Email
}