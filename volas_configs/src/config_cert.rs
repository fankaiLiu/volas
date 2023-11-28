use serde::Deserialize;
use std::path::Path;

use crate::loader_config::CFG;

#[derive(Debug, Deserialize)]
pub struct Cert {
    /// cert
    pub cert: String,
    /// key
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
pub fn get_cert_key() -> CertKey {
    let cert = get_string(&CFG.cert.cert);
    let key = get_string(&CFG.cert.key);
    CertKey::new(cert, key)
}

fn get_string<P: AsRef<Path>>(path: P) -> Vec<u8> {
    std::fs::read(path).expect("读取文件失败")
}
