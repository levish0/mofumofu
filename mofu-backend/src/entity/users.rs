use sea_orm::prelude::*;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    #[sea_orm(string_len = 20, not_null)]
    pub name: String,
    #[sea_orm(string_len = 20, not_null, unique)]
    pub handle: String, // Unique
    #[sea_orm(string_len = 254, not_null, unique)]
    pub email: String, // Unique
    #[sea_orm(column_type = "Text", not_null)]
    pub password: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub profile_image: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub banner_image: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // 작성한 포스트들
    #[sea_orm(
        has_many = "super::posts::Entity",
        from = "Column::Id",
        to = "super::posts::Column::UserId"
    )]
    Posts,

    // 작성한 댓글들
    #[sea_orm(
        has_many = "super::comments::Entity",
        from = "Column::Id",
        to = "super::comments::Column::UserId"
    )]
    Comments,

    // 리프레시 토큰들
    #[sea_orm(
        has_many = "super::user_refresh_tokens::Entity",
        from = "Column::Id",
        to = "super::user_refresh_tokens::Column::UserId"
    )]
    RefreshTokens,

    // 이 유저를 팔로우하는 사람들 (followers)
    #[sea_orm(
        has_many = "super::follows::Entity",
        from = "Column::Id",
        to = "super::follows::Column::FolloweeId"
    )]
    Followers,

    // 이 유저가 팔로우하는 사람들 (following)
    #[sea_orm(
        has_many = "super::follows::Entity",
        from = "Column::Id",
        to = "super::follows::Column::FollowerId"
    )]
    Following,
}

impl Related<super::posts::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Posts.def()
    }
}

impl Related<super::comments::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Comments.def()
    }
}

impl Related<super::user_refresh_tokens::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RefreshTokens.def()
    }
}

impl Related<super::follows::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Followers.def()
    }
}

// Linked 구현: 이 유저의 팔로워들 가져오기
#[derive(Debug, Clone)]
pub struct GetFollowersLink;

impl Linked for GetFollowersLink {
    type FromEntity = Entity;
    type ToEntity = super::users::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![
            Relation::Followers.def(),                // users -> follows (followers)
            super::follows::Relation::Follower.def(), // follows -> users (follower info)
        ]
    }
}

// Linked 구현: 이 유저가 팔로우하는 사람들 가져오기
#[derive(Debug, Clone)]
pub struct GetFollowingLink;

impl Linked for GetFollowingLink {
    type FromEntity = Entity;
    type ToEntity = super::users::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![
            Relation::Following.def(),                // users -> follows (following)
            super::follows::Relation::Followee.def(), // follows -> users (followee info)
        ]
    }
}

impl ActiveModelBehavior for ActiveModel {}
