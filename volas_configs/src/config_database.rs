use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub name: String,
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout: u64,
    pub idle_timeout: u64,
    pub sqlx_logging: bool,
}

#[derive(Debug, Deserialize)]
pub struct DataBase {
    pub configs: Vec<DatabaseConfig>,
}