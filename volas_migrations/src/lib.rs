use include_dir::{include_dir, Dir};
use surrealdb::{Connection, Surreal};
use surrealdb_migrator::{Migrations, M};
static MIGRATION_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/migrations");

pub async fn run_migrations<C: Connection>(db: &Surreal<C>) {
    let migrations = Migrations::from_directory(&MIGRATION_DIR).unwrap();
    migrations
        .to_latest(&db)
        .await
        .expect("run migrations error");
}
