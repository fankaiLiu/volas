use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Surrealdb {
    pub username: String,
    pub password: String,
    pub ns: String,
    pub db: String,
}
