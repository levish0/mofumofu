use sea_orm::prelude::*;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "follows")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,

    #[sea_orm(column_type = "Uuid", not_null)]
    pub follower_id: Uuid,

    #[sea_orm(column_type = "Uuid", not_null)]
    pub followee_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::FollowerId",
        to = "super::users::Column::Id",
        on_delete = "Cascade"
    )]
    Follower,

    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::FolloweeId",
        to = "super::users::Column::Id",
        on_delete = "Cascade"
    )]
    Followee,
}

#[derive(Debug, Clone)]
pub struct FollowerLink;

impl Linked for FollowerLink {
    type FromEntity = Entity;
    type ToEntity = super::users::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Relation::Follower.def()]
    }
}

#[derive(Debug, Clone)]
pub struct FolloweeLink;

impl Linked for FolloweeLink {
    type FromEntity = Entity;
    type ToEntity = super::users::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Relation::Followee.def()]
    }
}

impl ActiveModelBehavior for ActiveModel {}
