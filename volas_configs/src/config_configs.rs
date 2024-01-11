use serde::Deserialize;

use crate::{
    config_cert::Cert, config_jwt::Jwt, config_log::Log, config_server::Server,
    config_surrealdb::Surrealdb,
};

#[derive(Debug, Deserialize)]
pub struct Configs {
    pub server: Server,
    pub log: Log,
    pub surrealdb: Surrealdb,
    pub cert: Cert,
    pub jwt: Jwt,
}
