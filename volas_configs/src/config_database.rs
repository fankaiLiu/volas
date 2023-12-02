use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DataBase {
    pub database_urls: HashMap<String, String>,
}
