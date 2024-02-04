use anyhow::Result;
use chrono::{Duration, Utc};
use configs::loader_config::Configurable;
use configs::Configs;
use jsonwebtoken::{decode, Algorithm, DecodingKey, EncodingKey, Validation};
use salvo::jwt_auth::{ConstDecoder, CookieFinder, HeaderFinder, QueryFinder};
use salvo::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    username: String,
    pub user_id: String,
    exp: i64,
}

#[allow(dead_code)]
pub fn jwt_middleware() -> JwtAuth<JwtClaims, ConstDecoder> {
    let config = Configs::config();
    let auth_handler: JwtAuth<JwtClaims, _> = JwtAuth::new(ConstDecoder::from_secret(
        config.jwt.jwt_secret.to_owned().as_bytes(),
    ))
    .finders(vec![
        Box::new(HeaderFinder::new()),
        Box::new(QueryFinder::new("token")),
        Box::new(CookieFinder::new("jwt_token")),
    ])
    .force_passed(true);
    auth_handler
}

#[allow(dead_code)]
pub fn get_token(username: String, user_id: String) -> Result<(String, i64)> {
    let config = Configs::config();
    let exp = Utc::now() + Duration::seconds(config.jwt.jwt_exp);
    let claim = JwtClaims {
        username,
        user_id,
        exp: exp.timestamp(),
    };
    let token: String = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claim,
        &EncodingKey::from_secret(config.jwt.jwt_secret.as_bytes()),
    )?;
    Ok((token, exp.timestamp()))
}

#[allow(dead_code)]
pub fn decode_token(token: &str) -> bool {
    let config = Configs::config();
    let validation = Validation::new(Algorithm::HS256);
    decode::<JwtClaims>(
        token,
        &DecodingKey::from_secret(config.jwt.jwt_secret.as_bytes()),
        &validation,
    )
    .is_ok()
}
