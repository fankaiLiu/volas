use salvo::Router;
use web::Routers;

pub mod system_info;

pub struct System;

impl Routers for System {
    fn build(self) -> Vec<salvo::Router> {
        vec![Router::new()
            .path("system")
            .append(&mut system_info::SystemInfo.build())]
    }
}
