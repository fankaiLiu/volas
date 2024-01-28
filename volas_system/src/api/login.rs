
use crate::services::sys_user_service::UserService;
use common::app_response::{AppResponse, AppResult};
use common::{app_response::ResponseBuilder, Routers};
use salvo::Writer;
use salvo::{
    handler,
    oapi::{endpoint, extract::JsonBody},
    Response, Router,
};
#[endpoint]
async fn login(
    req: JsonBody<crate::models::sys_user::LoginUser>,
) -> AppResult<AppResponse<crate::models::sys_user::UserInfo>> {
    let user_service = crate::services::sys_user_service_impl::MyUserService::default();
    let res = user_service.login(&req.0).await;
    Ok(AppResponse(res))
}


pub struct Login;

impl Routers for Login {
    fn build(self) -> Vec<salvo::Router> {
        vec![
            Router::with_path("login").path("account")
                .post(login)
                .options(handler::empty()),
        ]
    }
}
