use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Posts::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Posts::Id)
                            .uuid() // UUID 타입, 고유 식별자
                            .not_null()
                            .primary_key() // PK 지정
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(ColumnDef::new(Posts::Content).string_len(2000).not_null())
                    .col(ColumnDef::new(Posts::AuthorId).uuid().not_null())
                    .col(ColumnDef::new(Posts::ReplyToId).uuid().null()) // 답글 대상 포스트
                    .col(
                        ColumnDef::new(Posts::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::cust("now()")),
                    )
                    .col(
                        ColumnDef::new(Posts::UpdatedAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(Posts::IsDeleted)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Posts::LikeCount)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(Posts::ReplyCount)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    // 작성자와의 외래키
                    .foreign_key(
                        ForeignKey::create()
                            .from(Posts::Table, Posts::AuthorId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    // 답글 관계 외래키 (자기 참조)
                    .foreign_key(
                        ForeignKey::create()
                            .from(Posts::Table, Posts::ReplyToId)
                            .to(Posts::Table, Posts::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // 기본 인덱스들
        manager
            .create_index(
                Index::create()
                    .name("idx_posts_author_id")
                    .table(Posts::Table)
                    .col(Posts::AuthorId)
                    .to_owned(),
            )
            .await?;

        // 답글 조회 최적화 인덱스
        manager
            .create_index(
                Index::create()
                    .name("idx_posts_reply_to_id")
                    .table(Posts::Table)
                    .col(Posts::ReplyToId)
                    .to_owned(),
            )
            .await?;

        // 특정 포스트의 답글들을 시간순으로 조회하는 복합 인덱스
        manager
            .create_index(
                Index::create()
                    .name("idx_posts_reply_created")
                    .table(Posts::Table)
                    .col(Posts::ReplyToId)
                    .col(Posts::CreatedAt)
                    .to_owned(),
            )
            .await?;

        // 타임라인 조회 최적화 (최신 포스트들)
        manager
            .create_index(
                Index::create()
                    .name("idx_posts_created_at")
                    .table(Posts::Table)
                    .col(Posts::CreatedAt)
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

#[derive(DeriveIden)]
enum Posts {
    Table,
    Id,
    AuthorId,
    ReplyToId,
    Content,
    CreatedAt,
    UpdatedAt,
    IsDeleted,
    LikeCount,
    ReplyCount,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}