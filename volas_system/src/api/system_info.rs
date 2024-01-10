use common::{app_response::ResponseBuilder, Routers};
use salvo::{handler, Response, Router};
#[handler]
async fn system_info(res: &mut Response) {
    let sys = utils::sysinfo::SystemInfo::gather();
    ResponseBuilder::with_data(sys).into_response(res);
}

pub struct SystemInfo;

impl Routers for SystemInfo {
    fn build(self) -> Vec<salvo::Router> {
        vec![Router::new()
            .path("info")
            .get(system_info)
            .options(handler::empty())]
    }
}
