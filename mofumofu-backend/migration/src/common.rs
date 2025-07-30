use sea_orm_migration::prelude::*;
use strum::EnumIter;

#[derive(DeriveIden, EnumIter)]
pub enum PostStatusEnums {
    #[sea_orm(iden = "post_status")]
    Table,
    #[sea_orm(iden = "draft")]
    Draft,
    #[sea_orm(iden = "auto_saved")]
    AutoSaved,
    #[sea_orm(iden = "published")]
    Published,
}
