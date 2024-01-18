use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Surrealdb {
    pub url: String,
    pub username: String,
    pub password: String,
    pub ns: String,
    pub db: String,
}
