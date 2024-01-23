use common::Routers;
use configs::{loader_config::Configurable, Configs};
use infra::{DbService, SurrealdbServiceImpl};
use migrations::MigrationService;
use salvo::{
    conn::TcpListener,
    cors::{self, AllowHeaders, AllowMethods, Cors},
    server::ServerHandle,
    Listener, Router, Server, Service,
};
use system::System;
use tokio::{signal, sync::oneshot};

#[tokio::main]
async fn main() {
    let config = Configs::config();
    dbg!(&config.server.name);
    SurrealdbServiceImpl::init().await.unwrap();
    SurrealdbServiceImpl::run_migrations().await.unwrap();
    let cors_handler = Cors::new()
        .allow_origin(cors::Any)
        .allow_methods(AllowMethods::any())
        .allow_headers(AllowHeaders::any())
        .into_handler();

    let service: Service = Router::with_hoop(cors_handler)
        .append(&mut System.build())
        .into();
    println!("Starting server at {}", &config.server.address);
    let acceptor = TcpListener::new(&config.server.address).bind().await;
    let server = Server::new(acceptor);
    let handle = server.handle();
    // Graceful shutdown the server
    tokio::spawn(shutdown_signal(handle));
    server.serve(service).await;
}

async fn shutdown_signal(handle: ServerHandle) {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => println!("ctrl_c signal received"),
        _ = terminate => println!("terminate signal received"),
    }
    handle.stop_graceful(std::time::Duration::from_secs(5));
}
