use crate::models::sys_user::USER_TABLE;

use super::sys_user_service::UserService;
use infra::{DbService, SurrealdbServiceImpl};
use serde::Deserialize;
use surrealdb::sql::Thing;

#[derive(Debug, Default)]
pub struct MyUserService;
#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

impl UserService for MyUserService {
    async fn add(
        &self,
        new_user: &crate::models::sys_user::SysUser,
    ) -> common::app_response::AppResult<crate::models::sys_user::SysUser> {
        let db: &surrealdb::Surreal<surrealdb::engine::remote::ws::Client> =
            SurrealdbServiceImpl::pool().await.unwrap();
        let sql = "
            INSERT INTO sys_user
            {{
                first_name: $first_name,
                last_name: $last_name,
                email: $email,
                password: crypto::argon2::generate($password),
            }}
            ";
        let mut result = db
            .query(sql)
            .bind(("first_name", &new_user.first_name))
            .bind(("last_name", &new_user.last_name))
            .bind(("email", &new_user.email))
            .bind(("password", &new_user.password))
            .await
            .unwrap();
        dbg!(&result);
        let created: Option<crate::models::sys_user::SysUser> = result.take(0).unwrap();
        dbg!(&created);
        //todo!()
        Ok(created.unwrap())
    }
}
