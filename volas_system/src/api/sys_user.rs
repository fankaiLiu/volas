use crate::services::sys_user_service::UserService;
use common::{app_response::ResponseBuilder, Routers};
use salvo::Writer;
use salvo::{
    handler,
    oapi::{endpoint, extract::JsonBody},
    Response, Router,
};
#[endpoint]
async fn add_user(req: JsonBody<crate::models::sys_user::SysUser>, res: &mut Response) {
    let user_service = crate::services::sys_user_service_impl::MyUserService::default();
    let res = user_service.add(&req.0).await.unwrap();
    todo!()
}

pub struct SystemUser;

impl Routers for SystemUser {
    fn build(self) -> Vec<salvo::Router> {
        vec![Router::new().post(add_user).options(handler::empty())]
    }
}
