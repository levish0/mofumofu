use sea_orm_migration::prelude::*;
use strum::IntoEnumIterator;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Drafts::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Drafts::Id)
                            .uuid() // UUID 타입, 고유 식별자
                            .not_null()
                            .primary_key() // PK 지정
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(ColumnDef::new(Drafts::PostId).uuid().null()) // 기존 포스트 수정시
                    .col(ColumnDef::new(Drafts::UserId).uuid().not_null())
                    .col(ColumnDef::new(Drafts::Title).string_len(200).null())
                    .col(ColumnDef::new(Drafts::Summary).string_len(500).null())
                    .col(ColumnDef::new(Drafts::Content).text().null())
                    .col(
                        ColumnDef::new(Drafts::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::cust("now()")),
                    )
                    .col(
                        ColumnDef::new(Drafts::UpdatedAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(Drafts::AutoSaveVersion)
                            .integer()
                            .not_null()
                            .default(1), // 자동저장 버전
                    )
                    // 작성자와의 외래키
                    .foreign_key(
                        ForeignKey::create()
                            .from(Drafts::Table, Drafts::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    // 기존 포스트와의 외래키 (수정시)
                    .foreign_key(
                        ForeignKey::create()
                            .from(Drafts::Table, Drafts::PostId)
                            .to(Posts::Table, Posts::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // 기존 포스트의 드래프트 조회
        manager
            .create_index(
                Index::create()
                    .name("idx_drafts_post_id")
                    .table(Drafts::Table)
                    .col(Drafts::PostId)
                    .to_owned(),
            )
            .await?;

        // 생성일 기준 정렬
        manager
            .create_index(
                Index::create()
                    .name("idx_drafts_created_at")
                    .table(Drafts::Table)
                    .col(Drafts::CreatedAt)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Drafts::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Drafts {
    Table,
    Id,
    PostId,
    UserId,
    Title,
    Summary,
    Content,
    CreatedAt,
    UpdatedAt,
    AutoSaveVersion,
}
#[derive(DeriveIden)]
enum Posts {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
