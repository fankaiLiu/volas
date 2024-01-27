use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("error:`{0}`")]
    AnyHow(#[from] anyhow::Error),
    #[error("surrealdb:`{0}`")]
    Surrealdb(#[from] surrealdb::Error),
    #[error("http::ParseError:`{0}`")]
    Parse(#[from] salvo::http::ParseError),
    #[error("external error: `{0}`")]
    External(#[from] Box<dyn std::error::Error + Send + Sync>),
}
impl Default for AppError {
    fn default() -> Self {
        AppError::AnyHow(anyhow::anyhow!("default error"))
    }
}
