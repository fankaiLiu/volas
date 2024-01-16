use common::Routers;
use configs::{loader_config::Configurable, Configs};
use infra::{DbService, SurrealdbServiceImpl};
use migrations::MigrationService;
use salvo::{
    conn::TcpListener,
    cors::{self, AllowHeaders, AllowMethods, Cors},
    Listener, Router, Server, Service,
};
use system::System;
use tokio::sync::oneshot;

#[tokio::main]
async fn main() {
    let config = Configs::config();
    dbg!(&config.server.name);
    SurrealdbServiceImpl::init().await.unwrap();
    SurrealdbServiceImpl::run_migrations().await;
    let cors_handler = Cors::new()
        .allow_origin(cors::Any)
        .allow_methods(AllowMethods::any())
        .allow_headers(AllowHeaders::any())
        .into_handler();

    let service: Service = Router::with_hoop(cors_handler)
        .append(&mut System.build())
        .into();
    println!("Starting server at {}", &config.server.address);
    let (tx, rx) = oneshot::channel();
    let acceptor = TcpListener::new(&config.server.address).bind().await;
    let server = Server::new(acceptor).serve_with_graceful_shutdown(
        service,
        async {
            rx.await.ok();
        },
        None,
    );
    tokio::task::spawn(server);
    tokio::signal::ctrl_c().await.unwrap();
    let _ = tx.send(());
}
