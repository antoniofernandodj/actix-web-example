use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(

                Table::create()

                    .table(User::Table)
                    .if_not_exists()

                    .col(
                        ColumnDef::new(User::Uuid)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(User::Name)
                            .string()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(User::Username)
                            .string()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(User::Email)
                            .string()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(User::PasswordHash)
                            .string()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(User::CreatedAt)
                            .date_time()
                            .not_null()
                    )
                    
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Uuid,
    Name,
    Username,
    Email,
    PasswordHash,
    CreatedAt,
}

// ALTER TABLE nome_da_tabela
// ADD CONSTRAINT nome_da_constrain UNIQUE (nome_da_coluna);