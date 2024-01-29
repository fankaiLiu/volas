use super::sys_user_service::UserService;
use common::{middleware::jwt::get_token, AppError};
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
                username: $username,
                email: $email,
                password: crypto::argon2::generate($password),
            }
            ";
        let mut result = db
            .query(sql)
            .bind(("username", &new_user.username))
            .bind(("email", &new_user.email))
            .bind(("password", &new_user.password))
            .await?;
        //dbg!(&result);
        let created: Option<crate::models::sys_user::SysUser> = result.take(0)?;
        Ok(created.unwrap().into())
    }

    async fn login(
        &self,
        login_user: &crate::models::sys_user::LoginUser,
    ) -> common::app_response::AppResult<crate::models::sys_user::UserInfo> {
        let db: &surrealdb::Surreal<surrealdb::engine::remote::ws::Client> =
            SurrealdbServiceImpl::pool().await.unwrap();
        let sql = format!("return crypto::argon2::compare((select password from sys_user where username= $username)[0].password,$password)");
        let mut result = db
            .query(&sql)
            .bind(("username", &login_user.username))
            .bind(("password", &login_user.password))
            .await?;
        let created: Option<bool> = result.take(0)?;
        match created {
            Some(true) => {
                let sql = format!("select * from sys_user where username= $username");
                let mut result = db
                    .query(&sql)
                    .bind(("username", &login_user.username))
                    .await?;
                let created: Option<crate::models::sys_user::SysUser> = result.take(0)?;
                let user= created.unwrap();
                //let token=get_token(user.email,user.id.id.to_string())?;
                Ok(user.into())
            }
            _ => {
                return Err(AppError::AnyHow(anyhow::anyhow!("password is invalid")))
            },
        }
    }
}
