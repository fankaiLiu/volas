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
        new_user: &crate::models::sys_user::SysUser<'_>,
    ) -> common::app_response::AppResult<crate::models::sys_user::SysUser> {
        let db = SurrealdbServiceImpl::pool().await.unwrap();
        let created: Vec<Record> = db.create("sys_user").content(new_user).await.unwrap();
        dbg!(created);
        todo!()
    }
}
