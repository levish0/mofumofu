pub use sea_orm_migration::prelude::*;
mod m20250717_064916_create_users;
mod m20250718_155828_user_refresh_tokens;
mod m20250718_162056_posts;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250717_064916_create_users::Migration),
            Box::new(m20250718_155828_user_refresh_tokens::Migration),
            Box::new(m20250718_162056_posts::Migration),
        ]
    }
}
