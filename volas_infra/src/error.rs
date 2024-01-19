use thiserror::Error;

pub type Result<T> = std::result::Result<T, InfraError>;

#[derive(Error, Debug)]
pub enum InfraError {
    #[error("SurrealdbError: `{0}`")]
    SurrealdbError(#[from] surrealdb::Error),
    #[error("Database connection not initialized")]
    DatabaseNotInitialized,
}
