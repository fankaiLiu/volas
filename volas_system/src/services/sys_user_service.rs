use common::app_response::AppResult;

use crate::models::sys_user::SysUser;

pub trait UserService: Sync + Send {
    async fn add(&self, new_user: &SysUser) -> AppResult<SysUser>;
}
