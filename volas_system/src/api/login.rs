use crate::services::sys_user_service::UserService;
use common::app_response::{AppResponse, AppResult, ErrorResponseBuilder};
use common::AppError;
use common::{app_response::ResponseBuilder, Routers};
use salvo::http::cookie::Cookie;
use salvo::{
    handler,
    oapi::{endpoint, extract::JsonBody},
    Response, Router,
};
use salvo::{Request, Writer};
#[endpoint]
async fn login(req: JsonBody<crate::models::sys_user::LoginUser>, res: &mut Response) {
    let user_service = crate::services::sys_user_service_impl::MyUserService::default();
    let result = user_service.login(&req.0).await;
    match result {
        Ok(data) => {
            let jwt_token = data.token.clone();
            let cookie = Cookie::build(("jwt_token", jwt_token))
                .path("/")
                .http_only(true)
                .build();
            res.add_cookie(cookie);
            let cookies = res.cookies();
            ResponseBuilder::with_data(data).into_response(res);
        }
        Err(e) => ErrorResponseBuilder::with_err(e).into_response(res),
    }
}

#[endpoint]
async fn current_user(req:&mut Request,res: &mut Response) {
    let cookies = req.cookies();
    dbg!(&cookies);
    let cookie = cookies.get("jwt_token");
    let jwt_token = cookie.map(|c| c.value().to_string());
    match jwt_token {
        Some(token) => {
            let user_service = crate::services::sys_user_service_impl::MyUserService::default();
            let result = user_service.current_user(token).await;
            match result {
                Ok(data) => ResponseBuilder::with_data(data).into_response(res),
                Err(e) => ErrorResponseBuilder::with_err(e).into_response(res),
            }
        }
        None => {
            let e = AppError::AnyHow(anyhow::anyhow!("token is invalid"));
            ErrorResponseBuilder::with_err(e).into_response(res)
        }
    }
}

pub struct Login;

impl Routers for Login {
    fn build(self) -> Vec<salvo::Router> {
        vec![
            Router::with_path("login")
                .path("account")
                .post(login)
                .options(handler::empty()),
            Router::new().path("currentUser").get(current_user)  .options(handler::empty()),
        ]
    }
}
