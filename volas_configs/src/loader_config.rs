use once_cell::sync::Lazy;
use std::{env, fs::File, io::Read};

use crate::{
    config_cert::{get_cert_key, CertKey},
    config_configs::Configs,
};

const CONFIG_FILE: &str = "config/config.toml";

pub static CFG: Lazy<Configs> = Lazy::new(|| {
    let args: Vec<String> = env::args().collect();
    let config_file = args.get(1).unwrap_or(&CONFIG_FILE.to_string()).clone();
    Configs::init(config_file)
});
pub static CERT_KEY: Lazy<CertKey> = Lazy::new(get_cert_key);

impl Configs {
    fn init(config_file: String) -> Self {
        let mut file = match File::open(&config_file) {
            Ok(f) => f,
            Err(e) => panic!(
                "Config file does not exist:{}, Error message:{}",
                CONFIG_FILE, e
            ),
        };
        let mut cfg_contents = String::new();
        match file.read_to_string(&mut cfg_contents) {
            Ok(s) => s,
            Err(e) => panic!("Failed to read the config file, Error message:{}", e),
        };
        match toml::from_str(&cfg_contents) {
            Ok(c) => c,
            Err(e) => panic!("Failed to parse the config file, Error message:{}", e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;

    #[test]
    fn test_init() {
        let config_file = "test_config.toml";
        let mut file = File::create(&config_file).unwrap();
        write!(
            file,
            r#"
        [server]
        name = "volas"
        address = "0.0.0.0:5800"
        ssl = false
        cors_allow_origin=["https://salvo.rs"]
        [database]
        [[database.configs]]
        name = "system"
        url = "postgres://liufankai:1@localhost:5432/volas_system"
        max_connections = 1000
        min_connections = 5
        connect_timeout = 8
        idle_timeout = 8
        sqlx_logging = false        
        [jwt]
        jwt_secret = "secret"
        jwt_exp = 6000
        [cert]
        cert = "config/certs/cert.pem"
        key = "config/certs/key.pem"
        [log]
        filter_level = "info"
        with_ansi = true
        to_stdout = true
        directory = "./logs"
        file_name = "my-service.log"
        rolling = "daily"
        "#
        )
        .unwrap();

        let configs = Configs::init(config_file.to_string());
        assert_eq!(configs.server.name, "volas");

        let _ = std::fs::remove_file(Path::new(config_file));
    }
}
