use sea_orm::{DeriveActiveEnum, EnumIter};

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "post_status")]
pub enum PostStatusEnums {
    #[sea_orm(string_value = "draft")]
    Draft,

    #[sea_orm(string_value = "auto_saved")]
    AutoSaved,

    #[sea_orm(string_value = "published")]
    Published,
}
