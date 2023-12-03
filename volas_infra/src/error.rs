use thiserror::Error;

pub type Result<T> = std::result::Result<T, InfraError>;

#[derive(Error, Debug)]
pub enum InfraError {
    #[error("database `{0}` config is not exist")]
    ConfigNotExist(String),
    #[error("unknown infra error")]
    Unknown,
}
