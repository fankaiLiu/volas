use common::Routers;
use salvo::Router;
pub mod login;
pub mod sys_user;
pub mod system_info;
pub struct System;

impl Routers for System {
    fn build(self) -> Vec<salvo::Router> {
        vec![
            Router::new()
                .path("system")
                .append(&mut system_info::SystemInfo.build()),
            Router::new()
                .path("users")
                .append(&mut sys_user::SystemUser.build()),
            Router::new().append(&mut login::Login.build()),
        ]
    }
}
