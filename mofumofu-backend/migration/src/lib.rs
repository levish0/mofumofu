pub use sea_orm_migration::prelude::*;
mod common;
mod m20250717_064916_create_users;
mod m20250718_155828_user_refresh_tokens;
mod m20250718_155829_oauth_providers;
mod m20250718_155830_user_oauth_connections;
mod m20250718_162056_hashtags;
mod m20250718_162057_posts;
mod m20250718_162058_drafts;
mod m20250718_162101_post_hashtags;
mod m20250718_162102_comments;
mod m20250719_031841_follows;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250717_064916_create_users::Migration),
            Box::new(m20250718_155828_user_refresh_tokens::Migration),
            Box::new(m20250718_155829_oauth_providers::Migration),
            Box::new(m20250718_155830_user_oauth_connections::Migration),
            Box::new(m20250718_162056_hashtags::Migration),
            Box::new(m20250718_162057_posts::Migration),
            Box::new(m20250718_162058_drafts::Migration),
            Box::new(m20250718_162101_post_hashtags::Migration),
            Box::new(m20250718_162102_comments::Migration),
            Box::new(m20250719_031841_follows::Migration),
        ]
    }
}
