use common::Routers;
use configs::CFG;
use infra::{init_db_conn, DATABASE_SERVICE};
use migrations::run_migrations;
use salvo::{
    conn::TcpListener,
    cors::{self, AllowHeaders, AllowMethods, Cors},
    Listener, Router, Server, Service,
};
use system::System;
use tokio::sync::oneshot;

#[tokio::main]
async fn main() {
    dbg!(&CFG.server.name);
    init_db_conn().await;
    run_migrations(DATABASE_SERVICE.get().unwrap()).await;
    let cors_handler = Cors::new()
        .allow_origin(cors::Any)
        .allow_methods(AllowMethods::any())
        .allow_headers(AllowHeaders::any())
        .into_handler();

    let service: Service = Router::with_hoop(cors_handler)
        .append(&mut System.build())
        .into();
    println!("Starting server at {}", &CFG.server.address);
    let (tx, rx) = oneshot::channel();
    let acceptor = TcpListener::new(&CFG.server.address).bind().await;
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
