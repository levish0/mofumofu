use sea_orm::{DeriveActiveEnum, EnumIter};

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "oauth_provider")]
pub enum OAuthProvider {
    #[sea_orm(string_value = "google")]
    Google,

    #[sea_orm(string_value = "github")]
    Github,
}
