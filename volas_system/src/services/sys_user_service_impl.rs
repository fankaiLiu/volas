use crate::models::sys_user::USER_TABLE;

use super::sys_user_service::UserService;
use common::AppError;
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
        new_user: &crate::models::sys_user::NewUser,
    ) -> common::app_response::AppResult<crate::models::sys_user::UserInfo> {
        let db: &surrealdb::Surreal<surrealdb::engine::remote::ws::Client> =
            SurrealdbServiceImpl::pool().await.unwrap();
        let sql = "
            CREATE sys_user CONTENT
            {
                first_name: $first_name,
                last_name: $last_name,
                email: $email,
                password: crypto::argon2::generate($password),
            }
            ";
        let mut result = db
            .query(sql)
            .bind(("first_name", &new_user.first_name))
            .bind(("last_name", &new_user.last_name))
            .bind(("email", &new_user.email))
            .bind(("password", &new_user.password))
            .await?;
        //dbg!(&result);
        let created: Option<crate::models::sys_user::SysUser> = result.take(0)?;
        dbg!(&created);
        //todo!()
        Ok(created.unwrap().into())
    }

    async fn login(
        &self,
        login_user: &crate::models::sys_user::LoginUser,
    ) -> common::app_response::AppResult<crate::models::sys_user::UserInfo> {
        let db: &surrealdb::Surreal<surrealdb::engine::remote::ws::Client> =
            SurrealdbServiceImpl::pool().await.unwrap();
        let sql = format!("return crypto::argon2::compare((select password from sys_user where email= $email)[0].password,$password)");
        let mut result = db
            .query(&sql)
            .bind(("email", &login_user.email))
            .bind(("password", &login_user.password))
            .await?;
        let created: Option<bool> = result.take(0)?;
        match created {
            Some(true) => {
                let sql = format!("select * from sys_user where email= $email");
                let mut result = db
                    .query(&sql)
                    .bind(("email", &login_user.email))
                    .await?;
                let created: Option<crate::models::sys_user::SysUser> = result.take(0)?;
                Ok(created.unwrap().into())
            }
            _ => {
                return Err(AppError::AnyHow(anyhow::anyhow!("password is invalid")))
            },
        }
    }
}
