use sea_orm::prelude::*;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "posts")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,

    #[sea_orm(column_type = "Text", not_null, string_len = 2000)]
    pub content: String,

    #[sea_orm(column_type = "Uuid", not_null)]
    pub post_id: Uuid,

    #[sea_orm(column_type = "Uuid", not_null)]
    pub user_id: Uuid,

    #[sea_orm(column_type = "Uuid", nullable)]
    pub parent_id: Option<Uuid>,

    #[sea_orm(column_type = "TimestampWithTimeZone", not_null)]
    pub created_at: DateTimeUtc,

    #[sea_orm(column_type = "TimestampWithTimeZone", nullable)]
    pub updated_at: Option<DateTimeUtc>,

    #[sea_orm(column_type = "Boolean", not_null, default_value = "false")]
    pub is_deleted: bool,

    #[sea_orm(column_type = "Integer", not_null, default_value = "0")]
    pub like_count: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
