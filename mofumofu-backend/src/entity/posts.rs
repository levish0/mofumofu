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

    #[sea_orm(column_type = "TimestampWithTimeZone", nullable)]
    pub deleted_at: Option<DateTimeUtc>,

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
    User,

    #[sea_orm(
        has_many = "super::comments::Entity",
        from = "Column::Id",
        to = "super::comments::Column::PostId"
    )]
    Comments,

    #[sea_orm(
        has_many = "super::post_hash_tags::Entity",
        from = "Column::Id",
        to = "super::post_hash_tags::Column::PostId"
    )]
    PostHashTags,

    // 관련된 임시저장들
    #[sea_orm(
        has_many = "super::drafts::Entity",
        from = "Column::Id",
        to = "super::drafts::Column::PostId"
    )]
    Drafts,
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::comments::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Comments.def()
    }
}

impl Related<super::post_hash_tags::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PostHashTags.def()
    }
}

impl Related<super::drafts::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Drafts.def()
    }
}

// Post -> HashTags (Many-to-Many through PostHashTags)
#[derive(Debug, Clone)]
pub struct PostToHashTagsLink;

impl Linked for PostToHashTagsLink {
    type FromEntity = Entity;
    type ToEntity = super::hash_tags::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![
            Relation::PostHashTags.def(),
            super::post_hash_tags::Relation::HashTag.def(),
        ]
    }
}

impl ActiveModelBehavior for ActiveModel {}
