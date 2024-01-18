use include_dir::{include_dir, Dir};
use infra::{DbService, SurrealdbServiceImpl};
use surrealdb::{Connection, Surreal};
use surrealdb_migrator::{Migrations, M};
static MIGRATION_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/migrations");

#[derive(Debug)]
pub enum MigrationError {
    LoadError(String),
    ApplyError(String),
}
pub trait MigrationService {
    async fn run_migrations() -> Result<(), MigrationError>;
}

impl MigrationService for SurrealdbServiceImpl {
    async fn run_migrations() -> Result<(), MigrationError> {
        let pool = Self::pool().await.unwrap();
        let migrations = Migrations::from_directory(&MIGRATION_DIR).unwrap();
        migrations
            .to_latest(&pool)
            .await
            .expect("Failed to run migrations");
        Ok(())
    }
}
