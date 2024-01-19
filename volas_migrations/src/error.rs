use thiserror::Error;

pub type Result<T> = std::result::Result<T, MigrationError>;

#[derive(Error, Debug)]
pub enum MigrationError {
    #[error("SurrealdbError: `{0}`")]
    SurrealdbError(#[from] surrealdb::Error),
}
