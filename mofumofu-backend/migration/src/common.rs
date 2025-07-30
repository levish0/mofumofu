use sea_orm_migration::prelude::*;
use strum::EnumIter;

#[derive(DeriveIden, EnumIter)]
pub enum OAuthProvider {
    #[sea_orm(iden = "oauth_provider")]
    Table,
    #[sea_orm(iden = "google")]
    Google,
    #[sea_orm(iden = "github")]
    Github,
}
