use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // PostgreSQL의 pgcrypto extension 설치 (UUID 자동생성 위해)
        manager
            .get_connection()
            .execute_unprepared("CREATE EXTENSION IF NOT EXISTS \"pgcrypto\";")
            .await?;

        // users 테이블 생성
        // - id: UUID, PK, 자동 생성 (gen_random_uuid)
        // - name: 유저 이름, 20자 제한, NOT NULL
        // - handle: 고유 핸들, 20자 제한, NOT NULL, UNIQUE (로그인/식별자)
        // - email: 이메일, 254자 제한, NOT NULL, UNIQUE
        // - password: 비밀번호 해시, NULL (plain-text 저장 금지)
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Users::Id)
                            .uuid() // UUID 타입, 고유 식별자
                            .not_null()
                            .primary_key() // PK 지정
                            .default(Expr::cust("gen_random_uuid()")), // 자동 생성
                    )
                    .col(string_len(Users::Name, 20).not_null()) // 이름, 20자 제한
                    .col(string_len(Users::Handle, 20).not_null().unique_key()) // 핸들, UNIQUE
                    .col(string_len(Users::Email, 254).not_null().unique_key()) // 이메일, UNIQUE
                    .col(ColumnDef::new(Users::Password).text().null()) // 비밀번호 해시, OAUTH 지원 시 NULL 가능
                    .col(
                        ColumnDef::new(Users::IsVerified)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(Users::ProfileImage).text().null())
                    .col(ColumnDef::new(Users::BannerImage).text().null())
                    .to_owned(),
            )
            .await?;

        // handle 컬럼 인덱스 생성 (로그인/검색 성능 최적화)
        manager
            .create_index(
                Index::create()
                    .name("idx_users_handle")
                    .table(Users::Table)
                    .col(Users::Handle)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // handle 인덱스 삭제 (테이블 삭제 전에 인덱스 먼저 삭제)
        manager
            .drop_index(
                Index::drop()
                    .name("idx_users_handle")
                    .table(Users::Table)
                    .to_owned(),
            )
            .await?;

        // users 테이블 삭제 (모든 유저 정보 삭제됨, 롤백 시 주의)
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
    Name,
    Handle,
    Email,
    Password,
    IsVerified,
    ProfileImage,
    BannerImage,
}
