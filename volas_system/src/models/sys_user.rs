use std::default;

use chrono::{DateTime, Local, Utc};
use salvo::oapi::{Object, Schema, ToSchema};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
pub const USER_TABLE: &str = "sys_user";

#[derive(Debug, Deserialize, Serialize)]
pub struct SysUser {
    pub id: surrealdb::sql::Thing,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<SysUser> for UserInfo {
    fn from(value: SysUser) -> Self {
        Self {
            id: value.id.id.to_string(),
            first_name: value.first_name,
            last_name: value.last_name,
            email: value.email,
            created_at: value.created_at.with_timezone(&Local),
            updated_at: value.updated_at.with_timezone(&Local),
        }
    }
}
#[derive(Debug, Deserialize, Serialize, ToSchema, Default)]
pub struct UserInfo {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema, Default)]
pub struct NewUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema, Default)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}
