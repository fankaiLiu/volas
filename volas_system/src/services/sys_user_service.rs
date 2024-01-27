use common::app_response::AppResult;

use crate::models::sys_user::{LoginUser, NewUser, UserInfo};

pub trait UserService: Sync + Send {
    async fn add(&self, new_user: &NewUser) -> AppResult<UserInfo>;
    async fn login(&self, login_user: &LoginUser) -> AppResult<UserInfo>;
}
