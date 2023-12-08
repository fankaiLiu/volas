use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("error:`{0}`")]
    AnyHow(#[from] anyhow::Error),
    #[error("http::ParseError:`{0}`")]
    ParseError(#[from] salvo::http::ParseError),
    #[error("external error: `{0}`")]
    ExternalError(#[from] Box<dyn std::error::Error + Send>),
}
