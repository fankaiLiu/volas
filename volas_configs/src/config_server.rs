use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Server {
    pub name: String,
    pub address: String,
    pub cors_allow_origin: Vec<String>,
    pub ssl: bool,
}
