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
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}