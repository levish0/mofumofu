use sea_orm::prelude::*;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "posts")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    #[sea_orm(column_type = "Uuid", not_null)]
    pub author_id: Uuid,
    #[sea_orm(column_type = "Uuid", nullable)]
    pub reply_to_id: Option<Uuid>,
    #[sea_orm(column_type = "Text", string_len = 2000)]
    pub content: String,
    #[sea_orm(column_type = "TimestampWithTimeZone")]
    pub created_at: DateTimeUtc,
    #[sea_orm(column_type = "TimestampWithTimeZone", nullable)]
    pub updated_at: Option<DateTimeUtc>,
    pub is_deleted: bool,
    pub like_count: i32,
    pub reply_count: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::AuthorId",
        to = "super::users::Column::Id",
        on_delete = "Cascade"
    )]
    Users,
    #[sea_orm(
        belongs_to = "Entity",
        from = "Column::ReplyToId",
        to = "Column::Id",
        on_delete = "Cascade"
    )]
    ReplyTo,
    #[sea_orm(has_many = "Entity")]
    Replies,
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Users.def()
    }
}

// 자기 참조 관계들
impl Related<Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ReplyTo.def()
    }

    fn via() -> Option<RelationDef> {
        Some(Relation::Replies.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
