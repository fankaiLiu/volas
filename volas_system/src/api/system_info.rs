use common::Routers;
use salvo::{handler, writing::Json, Router};
#[handler]
async fn system_info() -> Json<utils::sysinfo::SystemInfo> {
    let sys = utils::sysinfo::SystemInfo::gather();
    Json(sys)
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
