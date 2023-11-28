use serde::Deserialize;

use crate::{
    config_cert::Cert, config_database::DataBase, config_jwt::Jwt, config_log::Log,
    config_server::Server,
};

#[derive(Debug, Deserialize)]
pub struct Configs {
    pub server: Server,
    pub log: Log,
    pub database: DataBase,
    pub cert: Cert,
    pub jwt: Jwt,
}
