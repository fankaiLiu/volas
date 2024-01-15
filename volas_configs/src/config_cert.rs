use crate::loader_config::Configurable;
use crate::Configs;
use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Cert {
    pub cert: String,
    pub key: String,
}

pub struct CertKey {
    pub cert: Vec<u8>,
    pub key: Vec<u8>,
}

impl CertKey {
    pub fn new(cert: Vec<u8>, key: Vec<u8>) -> Self {
        Self { cert, key }
    }
}

pub trait CertProvider {
    fn load_certs(&self) -> Result<CertKey, CertError>;
}

#[derive(Debug)]
pub enum CertError {
    IoError(std::io::Error),
}

impl From<std::io::Error> for CertError {
    fn from(err: std::io::Error) -> Self {
        CertError::IoError(err)
    }
}

impl CertProvider for Configs {
    fn load_certs(&self) -> Result<CertKey, CertError> {
        let cert = fs::read(&self.cert.cert)?;
        let key = fs::read(&self.cert.key)?;
        Ok(CertKey::new(cert, key))
    }
}

fn get_string<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, CertError> {
    fs::read(path).map_err(CertError::from)
}

pub fn get_cert_key() -> CertKey {
    let config = Configs::config();
    let cert = get_string(&config.cert.cert).expect("Invalid certificate");
    let key = get_string(&config.cert.key).expect("Invalid certificate");
    CertKey::new(cert, key)
}
