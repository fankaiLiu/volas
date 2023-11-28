use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DataBase {
    pub database_url: String,
}
