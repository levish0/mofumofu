use crate::entity::common::PostStatusEnums;
use sea_orm::prelude::*;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "posts")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,

    #[sea_orm(column_type = "Text", not_null, string_len = 200)]
    pub title: String,

    #[sea_orm(column_type = "Text", nullable, string_len = 500)]
    pub summary: Option<String>,

    #[sea_orm(column_type = "Text", not_null)]
    pub content: String,

    #[sea_orm(column_type = "Uuid", not_null)]
    pub user_id: Uuid,

    #[sea_orm(column_type = "TimestampWithTimeZone", not_null)]
    pub created_at: DateTimeUtc,

    #[sea_orm(column_type = "TimestampWithTimeZone", nullable)]
    pub updated_at: Option<DateTimeUtc>,

    #[sea_orm(column_type = "Boolean", not_null, default_value = "false")]
    pub is_deleted: bool,

    #[sea_orm(column_name = "status", not_null)]
    pub status: PostStatusEnums,

    #[sea_orm(column_type = "TimestampWithTimeZone", nullable)]
    pub published_at: Option<DateTimeUtc>,

    #[sea_orm(column_type = "TimestampWithTimeZone", nullable)]
    pub last_auto_saved_at: Option<DateTimeUtc>,

    #[sea_orm(column_type = "Integer", not_null, default_value = "0")]
    pub like_count: i32,

    #[sea_orm(column_type = "Integer", not_null, default_value = "0")]
    pub comment_count: i32,

    #[sea_orm(column_type = "Integer", not_null, default_value = "0")]
    pub view_count: i32,

    #[sea_orm(column_type = "Text", nullable, string_len = 255)]
    pub slug: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::UserId",
        to = "super::users::Column::Id",
        on_delete = "Cascade"
    )]
    Users,
}

// Posts -> Users 관계 구현
impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Users.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
