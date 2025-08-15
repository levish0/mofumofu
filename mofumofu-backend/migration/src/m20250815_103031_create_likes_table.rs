use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Likes::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Likes::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(ColumnDef::new(Likes::UserId).uuid().not_null())
                    .col(ColumnDef::new(Likes::PostId).uuid().not_null())
                    .col(
                        ColumnDef::new(Likes::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::cust("CURRENT_TIMESTAMP")),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Likes::Table, Likes::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Likes::Table, Likes::PostId)
                            .to(Posts::Table, Posts::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_unique_user_post_like")
                    .table(Likes::Table)
                    .col(Likes::UserId)
                    .col(Likes::PostId)
                    .unique()
                    .to_owned(),
            )
            .await?;

        // user_id 단일 인덱스
        manager
            .create_index(
                Index::create()
                    .name("idx_likes_user_id")
                    .table(Likes::Table)
                    .col(Likes::UserId)
                    .to_owned(),
            )
            .await?;

        // post_id 단일 인덱스
        manager
            .create_index(
                Index::create()
                    .name("idx_likes_post_id")
                    .table(Likes::Table)
                    .col(Likes::PostId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Likes::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Likes {
    Table,
    Id,
    UserId,
    PostId,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Posts {
    Table,
    Id,
}